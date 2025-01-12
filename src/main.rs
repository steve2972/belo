mod cli;
mod config;
mod elo_system;

use cli::{Cli, Commands, VsResult};
use config::Config;
use elo_system::{EloSystem, GameResult};

use anyhow::{anyhow, Context, Result};
use clap::{Parser};
use dirs::home_dir;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};


fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { project_name } => init_project(&project_name)?,
        Commands::Activate { project_name } => activate_project(&project_name)?,
        Commands::List => list_projects()?,
        Commands::Whoami => whoami()?,
        Commands::Deactivate => deactivate()?,
        Commands::Delete { project_name } => delete_project(&project_name)?,
        Commands::Head { count } => {
            let elo_system = load_active_project()?;
            elo_system.print_top(count);
        }
        Commands::Add { id } => {
            let mut elo_system = load_active_project()?;
            elo_system.add_player(&id)?;
            save_active_project(&elo_system)?;
        }
        Commands::Vs { id1, result, id2 } => {
            let mut elo_system = load_active_project()?;
            let game_result = match result {
                VsResult::Gt => GameResult::Player1Wins,
                VsResult::Lt => GameResult::Player2Wins,
                VsResult::Eq => GameResult::Tie,
            };

            elo_system.record_game(&id1, game_result, &id2)?;
            save_active_project(&elo_system)?;
        }
        Commands::Info { id } => {
            let elo_system = load_active_project()?;
            elo_system.print_info(&id);
        }
    }

    Ok(())
}

fn init_project(project_name: &str) -> Result<()> {
    let projects_dir = get_projects_dir()?;
    let project_path = projects_dir.join(project_name);

    if project_path.exists() {
        return Err(anyhow!("Project '{}' already exists.", project_name));
    }

    fs::create_dir_all(&project_path)
        .with_context(|| format!("Failed to create project directory at '{:?}'", project_path))?;

    let elo_system = EloSystem::new();
    let elo_file = project_path.join("elo_data.json");
    let serialized = serde_json::to_string_pretty(&elo_system)?;
    let mut file = File::create(&elo_file)
        .with_context(|| format!("Failed to create Elo data file at '{:?}'", elo_file))?;
    file.write_all(serialized.as_bytes())?;

    println!("Project '{}' has been initialized.", project_name);
    // Activate the project after initialization
    activate_project(project_name)?;
    Ok(())
}

fn activate_project(project_name: &str) -> Result<()> {
    let projects_dir = get_projects_dir()?;
    let project_path = projects_dir.join(project_name);

    if !project_path.exists() {
        return Err(anyhow!("Project '{}' does not exist.", project_name));
    }

    let mut config = Config::load()?;
    config.active_project = Some(project_name.to_string());
    config.save()?;

    println!("Project '{}' has been activated.", project_name);
    Ok(())
}

fn list_projects() -> Result<()> {
    let projects_dir = get_projects_dir()?;

    if !projects_dir.exists() {
        println!("No projects found.");
        return Ok(());
    }

    let projects: Vec<_> = fs::read_dir(&projects_dir)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.file_name().into_string().ok().map(|s| {
                    if e.path().is_dir() {
                        s
                    } else {
                        format!("{} (file)", s)
                    }
                })
            })
        })
        .collect();

    if projects.is_empty() {
        println!("No projects found.");
    } else {
        println!("Projects:");
        for project in projects {
            println!("  - {}", project);
        }
    }

    Ok(())
}

fn whoami() -> Result<()> {
    let config = Config::load()?;
    if let Some(project_name) = &config.active_project {
        println!("Active project: {}", project_name);
    } else {
        println!("No active project.");
    }

    Ok(())
}

fn deactivate() -> Result<()> {
    let mut config = Config::load()?;
    if config.active_project.is_none() {
        println!("No active project to deactivate.");
    } else {
        config.active_project = None;
        config.save()?;

        println!("Project has been deactivated.");
    }
    Ok(())
}

fn delete_project(project_name: &str) -> Result<()> {
    // First deactivate the project if it's active
    deactivate()?;

    let projects_dir = get_projects_dir()?;
    let project_path = projects_dir.join(project_name);

    if !project_path.exists() {
        return Err(anyhow!("Project '{}' does not exist.", project_name));
    }

    fs::remove_dir_all(&project_path)
        .with_context(|| format!("Failed to delete project directory at '{:?}'", project_path))?;

    println!("Project '{}' has been deleted.", project_name);
    Ok(())
}

fn load_active_project() -> Result<EloSystem> {
    let config = Config::load()?;
    let active = config.active_project.ok_or_else(|| anyhow!("No active project. Please activate a project first"))?;

    let elo_path = get_projects_dir()?.join(&active).join("elo_data.json");

    if !elo_path.exists() {
        return Err(anyhow!("Elo data file not found for active project."));
    }

    let mut file = File::open(&elo_path)
        .with_context(|| format!("Failed to open Elo data file at '{:?}'", elo_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let elo_system: EloSystem = serde_json::from_str(&contents)
        .with_context(|| "Failed to parse Elo data file")?;

    Ok(elo_system)
}

fn save_active_project(elo_system: &EloSystem) -> Result<()> {
    let config = Config::load()?;
    let active = config.active_project.ok_or_else(|| anyhow!("No active project. Please activate a project first"))?;

    let elo_path = get_projects_dir()?.join(&active).join("elo_data.json");
    let serialized = serde_json::to_string_pretty(elo_system)?;
    let mut file = File::create(&elo_path)
        .with_context(|| format!("Failed to create Elo data file at '{:?}'", elo_path))?;

    file.write_all(serialized.as_bytes())?;
    Ok(())
}


fn get_projects_dir() -> Result<PathBuf> {
    let home = home_dir().ok_or_else(|| anyhow!("Failed to get home directory"))?;
    Ok(home.join(".cache").join("belo"))
}
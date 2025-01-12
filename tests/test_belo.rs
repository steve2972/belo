use assert_cmd::Command;
use indicatif::{ProgressBar, ProgressStyle};
use rand::{seq::SliceRandom, thread_rng, Rng};

#[test]
fn test_commands() {
    // 1. Initialize a new project
    let mut cmd_init = Command::cargo_bin("belo").unwrap();
    cmd_init.arg("init").arg("tmp").assert().success();

    // 2. Activate the project
    let mut cmd_activate = Command::cargo_bin("belo").unwrap();
    cmd_activate.arg("activate").arg("tmp").assert().success();

    // 3. Add two players
    let mut cmd_add_player1 = Command::cargo_bin("belo").unwrap();
    cmd_add_player1.arg("add").arg("test_player1");
    cmd_add_player1.assert().success();
    let mut cmd_add_player2 = Command::cargo_bin("belo").unwrap();
    cmd_add_player2.arg("add").arg("test_player2");
    cmd_add_player2.assert().success();

    // 4. Record a match: test_player1 > test_player2
    let mut cmd_vs = Command::cargo_bin("belo").unwrap();
    cmd_vs.arg("vs").arg("test_player1").arg("gt").arg("test_player2");
    cmd_vs.assert().success();

    // 5. Deactivate the project
    let mut cmd_deactivate = Command::cargo_bin("belo").unwrap();
    cmd_deactivate.arg("deactivate").assert().success();

    // 6. Remove the project
    let mut cmd_delete = Command::cargo_bin("belo").unwrap();
    cmd_delete.arg("delete").arg("tmp").assert().success();
}

#[test]
fn test_list_projects_command() {
    let mut cmd_list = Command::cargo_bin("belo").unwrap();
    cmd_list.arg("list").assert().success();
}

#[test]
fn test_whoami_with_no_active_project() {
    let mut cmd_whoami = Command::cargo_bin("belo").unwrap();
    cmd_whoami.arg("whoami").assert().success();
}

#[test]
fn test_activate_nonexistent_project() {
    let mut cmd_activate = Command::cargo_bin("belo").unwrap();
    cmd_activate.arg("activate").arg("nonexistent").assert().failure();
}

#[test]
fn test_big_load() {
    let mut cmd_init = Command::cargo_bin("belo").unwrap();
    cmd_init.arg("init").arg("tmp").assert().success();

    // Add 500 players with progress bar
    let bar = ProgressBar::new(500);
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
        .expect("Failed to create progress bar style");

    bar.set_style(style.clone());

    for i in 0..500 {
        let mut cmd_add_player = Command::cargo_bin("belo").unwrap();
        cmd_add_player.arg("add").arg(format!("test_player{}", i));
        cmd_add_player.assert().success();
        bar.inc(1); // Increment progress bar
    }
    bar.finish_with_message("Players added!");

    // Record 2500 matches with progress bar
    let bar = ProgressBar::new(2500);
    bar.set_style(style.clone());

    let mut rng = thread_rng();
    let outcomes = ["gt", "lt", "eq"];

    for _ in 0..2500 {
        let i = rng.gen_range(1..=500);
        let j = rng.gen_range(1..=500);
        let outcome = outcomes.choose(&mut rng).unwrap();
        let mut cmd_vs = Command::cargo_bin("belo").unwrap();
        cmd_vs
            .arg("vs")
            .arg(format!("test_player{}", i))
            .arg(*outcome)
            .arg(format!("test_player{}", j));
        cmd_vs.assert().success();
        bar.inc(1); // Increment progress bar
    }
    bar.finish_with_message("Matches recorded!");

    let mut cmd_delete = Command::cargo_bin("belo").unwrap();
    cmd_delete.arg("delete").arg("tmp").assert().success();

    // Check if the project is deleted
    let mut cmd_activate = Command::cargo_bin("belo").unwrap();
    cmd_activate.arg("activate").arg("tmp").assert().failure();
}
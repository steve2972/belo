use clap::{ValueEnum};
use anyhow::{Result};
use serde::{Deserialize, Serialize};
use prettytable::{Table, Row, Cell, format};
use skillratings::{
    glicko2::{glicko2, Glicko2Rating, Glicko2Config},
    Outcomes,
};

/// Individual player data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: String,
    pub wins: u32,
    pub losses: u32,
    pub ties: u32,
    rating: Glicko2Rating,
}


#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum GameResult {
    Player1Wins,
    Player2Wins,
    Tie,
}


/// A container for storing all relevant Elo system data.
#[derive(Debug, Serialize, Deserialize)]
pub struct EloSystem {
    players: Vec<Player>,
}

impl EloSystem {
    /// Create a new Elo system data structure.
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }

    /// Adds a new player to the system with a default rating of 1000.
    pub fn add_player(&mut self, id: &str) -> Result<()> {
        if self.players.iter().any(|player| player.id == id) {
            println!("Player with ID '{}' already exists.", id);
            return Ok(());
        }

        self.players.push(Player {
            id: id.to_string(),
            wins: 0,
            losses: 0,
            ties: 0,
            rating: Glicko2Rating::new(),
        });

        println!("Player with ID '{}' added.", id);
        Ok(())
    }

    /// Handle a matchup between two players.
    pub fn record_game(&mut self, id1: &str, result: GameResult, id2: &str) -> Result<()> {
        // Find the indices of both players
        let index1 = self.players.iter().position(|p| p.id == id1);
        let index2 = self.players.iter().position(|p| p.id == id2);

        if index1.is_none() || index2.is_none() {
            println!("Both players must exist before recording a game.");
            return Ok(());
        }

        let index1 = index1.unwrap();
        let index2 = index2.unwrap();

        // Ensure we don't borrow the same player twice
        if index1 == index2 {
            println!("A player cannot play against themselves.");
            return Ok(());
        }

        // Borrow players mutably without overlapping
        let (first, second) = if index1 < index2 {
            self.players.split_at_mut(index2)
        } else {
            self.players.split_at_mut(index1)
        };

        let player1 = &mut first[index1];
        let player2 = if index1 < index2 {
            &mut second[0]
        } else {
            &mut second[index2 - index1]
        };

        // Update stats based on game result
        let outcome = match result {
            GameResult::Player1Wins => {
                player1.wins += 1;
                player2.losses += 1;
                Outcomes::WIN
            }
            GameResult::Player2Wins => {
                player1.losses += 1;
                player2.wins += 1;
                Outcomes::LOSS
            }
            GameResult::Tie => {
                player1.ties += 1;
                player2.ties += 1;
                Outcomes::DRAW
            }
        };

        // let (new_elo1, new_elo2) = bayesian_elo_update(player1.elo, player2.elo, outcome);
        let config = Glicko2Config::new();
        let player1_old_elo = player1.rating.rating;
        let player2_old_elo = player2.rating.rating;

        let (new_player1, new_player2) = glicko2(
            &player1.rating,
            &player2.rating,
            &outcome,
            &config,
        );
        player1.rating = new_player1;
        player2.rating = new_player2;

        println!("Game recorded!");
        let player1_id = player1.id.clone();
        let player2_id = player2.id.clone();
        let new_elo1 = player1.rating.rating;
        let new_elo2 = player2.rating.rating;

        println!(
            "Player {} ELO: {:.2} -> {:.2} (Δ{:.2})",
            player1_id,
            player1_old_elo,
            new_elo1,
            new_elo1 - player1_old_elo
        );
        println!(
            "Player {} ELO: {:.2} -> {:.2} (Δ{:.2})",
            player2_id,
            player2_old_elo,
            new_elo2,
            new_elo2 - player2_old_elo
        );
        Ok(())
    }

    /// Print the top N players in the system.
    pub fn print_top(&self, n: Option<usize>) {
        let num = n.unwrap_or(5);

        // Sort by ELO descending
        let mut sorted_players = self.players.clone();
        sorted_players.sort_by(|a, b| b.rating.rating.partial_cmp(&a.rating.rating).unwrap());

        let mut table = Table::new();
        table.set_titles(Row::new(vec![
            Cell::new("Player ID").style_spec("Fb"),
            Cell::new("ELO").style_spec("Fb"),
            Cell::new("Wins").style_spec("Fb"),
            Cell::new("Losses").style_spec("Fb"),
            Cell::new("Ties").style_spec("Fb"),
        ]));

        for player in sorted_players.iter().take(num) {
            table.add_row(Row::new(vec![
                Cell::new(&player.id),
                Cell::new(&format!("{:.1}", player.rating.rating)),
                Cell::new(&player.wins.to_string()),
                Cell::new(&player.losses.to_string()),
                Cell::new(&player.ties.to_string()),
            ]));
        }

        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        table.printstd();
    }

    /// Print a specific player's stats.
    pub fn print_info(&self, id: &str) {
        if let Some(player) = self.players.iter().find(|player| player.id == id) {
            println!(
                "Player: {} | ELO: {} | Wins: {} | Losses: {} | Ties: {}",
                player.id, player.rating.rating.round() as i64, player.wins, player.losses, player.ties
            );
        } else {
            println!("Player with ID '{}' not found.", id);
        }
    }
}


use clap::{ValueEnum};
use anyhow::{Result};
use serde::{Deserialize, Serialize};
use prettytable::{Table, Row, Cell, format};
use skillratings::{
    glicko2::{glicko2, Glicko2Rating, Glicko2Config},
    Outcomes,
};
use std::collections::HashMap;

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
    players: HashMap<String, Player>,
}

impl EloSystem {
    /// Create a new Elo system data structure.
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    /// Adds a new player to the system with a default rating of 1000.
    pub fn add_player(&mut self, id: &str) -> Result<()> {
        if id.trim().is_empty() {
            println!("Player ID cannot be empty.");
            return Ok(());
        }
        if self.players.contains_key(id) {
            println!("Player with ID '{}' already exists.", id);
            return Ok(());
        }

        self.players.insert(
            id.to_string(),
            Player {
                id: id.to_string(),
                wins: 0,
                losses: 0,
                ties: 0,
                rating: Glicko2Rating::new(),
            },
        );

        println!("Player with ID '{}' added.", id);
        Ok(())
    }

    /// Handle a matchup between two players.
    pub fn record_game(&mut self, id1: &str, result: GameResult, id2: &str) -> Result<()> {
        if !self.players.contains_key(id1) || !self.players.contains_key(id2) {
            println!("Both players must exist before recording a game.");
            return Ok(());
        }

        if id1 == id2 {
            println!("A player cannot play against themselves.");
            return Ok(());
        }

        // Update player stats using a helper function
        self.update_stats(id1, result, true);
        self.update_stats(id2, result, false);

        // Handle Elo rating updates
        let config = Glicko2Config::new();
        let player1_old_elo = self.players[id1].rating.rating;
        let player2_old_elo = self.players[id2].rating.rating;

        let outcome = match result {
            GameResult::Player1Wins => Outcomes::WIN,
            GameResult::Player2Wins => Outcomes::LOSS,
            GameResult::Tie => Outcomes::DRAW,
        };

        let (new_player1, new_player2) = glicko2(
            &self.players[id1].rating,
            &self.players[id2].rating,
            &outcome,
            &config,
        );
        self.players.get_mut(id1).unwrap().rating = new_player1;
        self.players.get_mut(id2).unwrap().rating = new_player2;

        println!("Game recorded!");
        let player1_id = id1.to_string();
        let player2_id = id2.to_string();
        let new_elo1 = new_player1.rating;
        let new_elo2 = new_player2.rating;

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

    // New helper function to update stats
    fn update_stats(&mut self, id: &str, result: GameResult, is_player1: bool) {
        let player = self.players.get_mut(id).unwrap();
        match result {
            GameResult::Player1Wins => {
                if is_player1 {
                    player.wins += 1;
                } else {
                    player.losses += 1;
                }
            }
            GameResult::Player2Wins => {
                if is_player1 {
                    player.losses += 1;
                } else {
                    player.wins += 1;
                }
            }
            GameResult::Tie => {
                player.ties += 1;
            }
        }
    }

    /// Print the top N players in the system.
    pub fn print_top(&self, n: Option<usize>) {
        let num = n.unwrap_or(5);

        // Sort by ELO descending
        let mut sorted_players: Vec<_> = self.players.values().cloned().collect();
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
        if let Some(player) = self.players.get(id) {
            println!(
                "Player: {} | ELO: {} | Wins: {} | Losses: {} | Ties: {}",
                player.id, player.rating.rating.round() as i64, player.wins, player.losses, player.ties
            );
        } else {
            println!("Player with ID '{}' not found.", id);
        }
    }

    pub fn get_top_n(&self, n: Option<usize>) -> Vec<(String, f64, u32, u32, u32)> {
        let num = n.unwrap_or(5);
        let mut sorted_players: Vec<_> = self.players.values().cloned().collect();
        sorted_players.sort_by(|a, b| b.rating.rating.partial_cmp(&a.rating.rating).unwrap());
        sorted_players
            .into_iter()
            .take(num)
            .map(|p| (p.id, p.rating.rating, p.wins, p.losses, p.ties))
            .collect()
    }
}


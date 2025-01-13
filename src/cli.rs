use clap::{ValueEnum, Parser, Subcommand};

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum VsResult {
    /// Player1 wins
    Gt, // Greater Than ('gt')
    /// Player2 wins
    Lt, // Less Than ('lt')
    /// The match is a tie
    Eq, // Equal ('eq')
}

/// Main CLI entrypoint
#[derive(Parser, Debug)]
#[command(
    name = "belo",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = "Command line tool to manage Elo ratings",
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initializes a new project at the given project name.
    ///
    /// Creates a hidden directory to store player ratings and configurations.
    ///
    /// **Example**
    /// ```sh
    /// belo init my_project
    /// ```
    Init {
        project_name: String,
    },
    /// Activates an existing project by name.
    Activate {
        project_name: String,
    },
    /// Lists all existing projects.
    List,
    /// Shows the currently active project.
    Whoami,
    /// Deactivates the currently active project.
    Deactivate,
    /// Deletes a project by name.
    Delete {
        project_name: String,
    },
    /// Shows top N rated players
    Head {
        count: Option<usize>,
    },
    /// Adds a new player to the system
    Add {
        id: String,
    },
    /// Records a game between two players
    ///
    /// **Example**
    /// ```sh
    /// belo vs player1 gt player2
    /// ```
    Vs {
        id1: String,
        /// gt = Player1 wins, lt = Player2 wins, eq = Tie
        #[arg(value_enum)]
        result: VsResult,
        id2: String,
    },
    /// Shows the stats for a specific player
    Info {
        id: String,
    },
}

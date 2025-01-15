use pyo3::prelude::*;
use crate::elo_system::{EloSystem, GameResult};
use anyhow::Result; // to match existing error handling

#[pyclass]
struct PyEloSystem {
    system: EloSystem,
}

#[pymethods]
impl PyEloSystem {
    #[new]
    fn new() -> Self {
        Self {
            system: EloSystem::new(),
        }
    }

    fn add_player(&mut self, id: &str) -> PyResult<()> {
        self.system.add_player(id)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    fn record_game(&mut self, id1: &str, result: &str, id2: &str) -> PyResult<()> {
        let game_result = match result {
            "gt" => GameResult::Player1Wins,
            ">" => GameResult::Player1Wins,
            "lt" => GameResult::Player2Wins,
            "<" => GameResult::Player2Wins,
            "eq" => GameResult::Tie,
            "==" => GameResult::Tie,
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid game result")),
        };
        self.system.record_game(id1, game_result, id2)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    fn print_top(&self, n: Option<usize>) {
        self.system.print_top(n);
    }

    fn print_info(&self, id: &str) {
        self.system.print_info(id);
    }
}

#[pymodule]
fn belo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyEloSystem>()?;
    Ok(())
}
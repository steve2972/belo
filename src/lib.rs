mod elo_system;

use pyo3::prelude::*;
use crate::elo_system::{EloSystem, GameResult};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static GLOBAL_ELO_SYSTEM: Lazy<Mutex<EloSystem>> = Lazy::new(|| Mutex::new(EloSystem::new()));

#[pyfunction]
fn init_state() -> PyResult<()> {
    let mut sys = GLOBAL_ELO_SYSTEM.lock().unwrap();
    *sys = EloSystem::new();
    Ok(())
}

#[pyfunction]
fn add_player(id: &str) -> PyResult<()> {
    let mut sys = GLOBAL_ELO_SYSTEM.lock().unwrap();
    sys.add_player(id)
       .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

#[pyfunction]
fn record_game(id1: &str, result: &str, id2: &str) -> PyResult<()> {
    let game_result = match result {
        "gt" | ">"  => GameResult::Player1Wins,
        "lt" | "<"  => GameResult::Player2Wins,
        "eq" | "==" => GameResult::Tie,
        _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid game result")),
    };
    let mut sys = GLOBAL_ELO_SYSTEM.lock().unwrap();
    sys.record_game(id1, game_result, id2)
       .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

#[pyfunction]
#[pyo3(signature = (n=None))]
fn head(n: Option<usize>) {
    let sys = GLOBAL_ELO_SYSTEM.lock().unwrap();
    sys.print_top(n);
}

#[pyfunction]
fn print_info(id: &str) {
    let sys = GLOBAL_ELO_SYSTEM.lock().unwrap();
    sys.print_info(id);
}

#[pymodule]
fn belo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_state, m)?)?;
    m.add_function(wrap_pyfunction!(add_player, m)?)?;
    m.add_function(wrap_pyfunction!(record_game, m)?)?;
    m.add_function(wrap_pyfunction!(head, m)?)?;
    m.add_function(wrap_pyfunction!(print_info, m)?)?;
    Ok(())
}
use assert_cmd::Command;
use indicatif::{ProgressBar, ProgressStyle};
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

struct TestEnv {
    name: String,
}

impl TestEnv {
    fn new(test_name: &str) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let env_name = format!("{}_{}", test_name, now);
        let mut cmd_init = Command::cargo_bin("belo").unwrap();
        cmd_init.arg("init").arg(&env_name).assert().success();
        Self { name: env_name }
    }
    fn activate(&self) {
        let mut cmd_activate = Command::cargo_bin("belo").unwrap();
        cmd_activate.arg("activate").arg(&self.name).assert().success();
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        let mut cmd_delete = Command::cargo_bin("belo").unwrap();
        let _ = cmd_delete.arg("delete").arg(&self.name).assert();
    }
}

#[test]
fn test_commands() {
    let env = TestEnv::new("test_commands");
    env.activate();

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
}

#[test]
fn test_list_projects_command() {
    let env = TestEnv::new("test_list_projects_command");
    env.activate();
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
fn test_unexpected_index() {
    let env = TestEnv::new("test_unexpected_index");
    env.activate();

    // Add four users
    let mut cmd_add_player1 = Command::cargo_bin("belo").unwrap();
    cmd_add_player1.arg("add").arg("test_player1");
    cmd_add_player1.assert().success();
    let mut cmd_add_player2 = Command::cargo_bin("belo").unwrap();
    cmd_add_player2.arg("add").arg("test_player2");
    cmd_add_player2.assert().success();
    let mut cmd_add_player3 = Command::cargo_bin("belo").unwrap();
    cmd_add_player3.arg("add").arg("test_player3");
    cmd_add_player3.assert().success();
    let mut cmd_add_player4 = Command::cargo_bin("belo").unwrap();
    cmd_add_player4.arg("add").arg("test_player4");
    cmd_add_player4.assert().success();

    // Record a match that throws an error (version 0.1.0)
    let mut cmd_vs1 = Command::cargo_bin("belo").unwrap();
    cmd_vs1.arg("vs").arg("test_player1").arg("gt").arg("test_player2");
    cmd_vs1.assert().success();
    let mut cmd_vs2 = Command::cargo_bin("belo").unwrap();
    cmd_vs2.arg("vs").arg("test_player2").arg("gt").arg("test_player3");
    cmd_vs2.assert().success();
    let mut cmd_vs3 = Command::cargo_bin("belo").unwrap();
    cmd_vs3.arg("vs").arg("test_player3").arg("gt").arg("test_player4");
    cmd_vs3.assert().success();
    // This command failed before
    let mut cmd_vs4 = Command::cargo_bin("belo").unwrap();
    cmd_vs4.arg("vs").arg("test_player4").arg("gt").arg("test_player1");
    cmd_vs4.assert().success();
}

#[test]
fn test_big_load() {
    let env = TestEnv::new("test_big_load");
    env.activate();

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
    let bar = ProgressBar::new(100);
    bar.set_style(style.clone());

    let mut rng = thread_rng();
    let outcomes = ["gt", "lt", "eq"];

    for _ in 0..100 {
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
}
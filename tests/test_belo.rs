use assert_cmd::Command;

#[test]
fn test_commands() {
    // 1. Initialize a new project
    let mut cmd_init = Command::cargo_bin("belo").unwrap();
    cmd_init.arg("init").arg("tests/tmp_project").assert().success();

    // 2. Activate the project
    let mut cmd_activate = Command::cargo_bin("belo").unwrap();
    cmd_activate.arg("activate").arg("tests/tmp_project").assert().success();

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
    cmd_delete.arg("delete").arg("tests/tmp_project").assert().success();
}
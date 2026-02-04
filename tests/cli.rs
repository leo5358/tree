use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, File};
use tempfile::TempDir;

/// 輔助函式：建立一個測試用的目錄結構
/// root
/// ├── file1.txt
/// ├── main.rs
/// ├── .hidden
/// └── src
///     └── lib.rs
fn setup_test_env() -> TempDir {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    File::create(root.join("file1.txt")).unwrap();
    File::create(root.join("main.rs")).unwrap();
    File::create(root.join(".hidden")).unwrap(); // 隱藏檔
    
    let src = root.join("src");
    fs::create_dir(&src).unwrap();
    File::create(src.join("lib.rs")).unwrap();

    temp
}

#[test]
fn test_basic_output() {
    let temp = setup_test_env();
    let mut cmd = Command::cargo_bin("tree").unwrap();

    // 預設情況下應顯示一般檔案與目錄
    cmd.arg(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("main.rs"))
        .stdout(predicate::str::contains("src"));
}

#[test]
fn test_flag_all_shows_hidden() {
    let temp = setup_test_env();
    
    // 1. 預設不顯示隱藏檔
    let mut cmd = Command::cargo_bin("tree").unwrap();
    cmd.arg(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(".hidden").not());

    // 2. 加上 -a 應顯示隱藏檔
    let mut cmd_all = Command::cargo_bin("tree").unwrap();
    cmd_all.arg(temp.path())
        .arg("-a") // 或 --all
        .assert()
        .success()
        .stdout(predicate::str::contains(".hidden"));
}

#[test]
fn test_flag_depth_limits_output() {
    let temp = setup_test_env();

    // 限制深度為 1，不應該看到 src/lib.rs (深度為 2)
    let mut cmd = Command::cargo_bin("tree").unwrap();
    cmd.arg(temp.path())
        .args(&["-d", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("src"))      // 應該看到第一層目錄
        .stdout(predicate::str::contains("lib.rs").not()); // 不該看到第二層檔案
}

#[test]
fn test_flag_exclude_ignores_files() {
    let temp = setup_test_env();

    // 排除 main.rs
    let mut cmd = Command::cargo_bin("tree").unwrap();
    cmd.arg(temp.path())
        .args(&["-e", "main.rs"])
        .assert()
        .success()
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("main.rs").not()); // main.rs 應被隱藏
}

#[test]
fn test_flag_size_shows_bytes() {
    let temp = setup_test_env();

    // 加上 -s 應顯示大小格式 (例如 "0 B" 或 "[")
    let mut cmd = Command::cargo_bin("tree").unwrap();
    cmd.arg(temp.path())
        .arg("-s")
        .assert()
        .success()
        .stdout(predicate::str::contains("B")); // 應包含 Byte 單位
}

#[test]
fn test_flag_no_icon_removes_icons() {
    let temp = setup_test_env();

    // 1. 預設有圖示 (檢查 Rust 圖示 unicode: \u{e7a8})
    let mut cmd = Command::cargo_bin("tree").unwrap();
    cmd.arg(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{e7a8}")); 

    // 2. 加上 -n 應移除圖示
    let mut cmd_no_icon = Command::cargo_bin("tree").unwrap();
    cmd_no_icon.arg(temp.path())
        .arg("-n")
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{e7a8}").not()); // 不應包含圖示
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("tree").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust Tree CLI Tool")) // 來自 struct 定義
        .stdout(predicate::str::contains("--no-icon")); // 檢查新功能說明是否存在
}
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    run_command(command);
}

fn run_command(command: &str) {
    match command {
        "init" => init(),
        "uninstall" => uninstall(),

        _ => println!("Command not found"),
    }
}

fn init() {
    let pre_commit_dir = path::Path::new(".pre-commit");

    if !path::Path::new(".git").exists() {
        panic!("git cannot be found");
    }

    Command::new("git")
        .arg("config")
        .arg("core.hooksPath")
        .arg(pre_commit_dir)
        .spawn()
        .expect("Failed to set hooks path");

    fs::create_dir_all(pre_commit_dir).expect("Failed to create .pre-commit directory");

    let mut file = fs::File::create(pre_commit_dir.join("pre-commit"))
        .expect("Failed to create pre-commit file");

    file.write_all(b"#!/usr/bin/env sh\n# Run pre-commit hooks\n\nexit 0")
        .expect("Failed to write to pre-commit file");
}

fn uninstall() {
    let pre_commit_dir = path::Path::new(".pre-commit");

    if !path::Path::new(".git").exists() {
        panic!("git cannot be found");
    }

    Command::new("git")
        .arg("config")
        .arg("--unset")
        .arg("core.hooksPath")
        .spawn()
        .expect("Failed to unset hooks path");

    fs::remove_dir_all(pre_commit_dir).expect("Failed to remove .pre-commit directory");
}

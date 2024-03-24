use std::env;
use std::fs;
use std::io::prelude::*;
use std::os::unix::prelude::OpenOptionsExt;
use std::path;
use std::process::Command;

const ALLOWED_HOOKS: [&str; 13] = [
    "applypatch-msg",
    "commit-msg",
    "post-update",
    "pre-applypatch",
    "pre-commit",
    "pre-merge-commit",
    "pre-push",
    "pre-rebase",
    "pre-receive",
    "prepare-commit-msg",
    "push-to-checkout",
    "sendemail-validate",
    "update",
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();

        return;
    }

    let command = &args[1];
    let args = args[2..].to_vec();

    run_command(command, args);
}

fn run_command(command: &str, args: Vec<String>) {
    match command {
        "init" => init(args.contains(&"--no-pre-commit".to_string())),
        "uninstall" => uninstall(),
        "add" => add_hook(&args[0]),

        _ => println!("Command not found"),
    }
}

fn init(no_pre_commit: bool) {
    let pre_commit_dir = path::Path::new(".hooky");

    if !path::Path::new(".git").exists() {
        println!("[error] git cannot be found");

        return;
    }

    Command::new("git")
        .arg("config")
        .arg("core.hooksPath")
        .arg(pre_commit_dir)
        .spawn()
        .expect("Failed to set hooks path");

    println!("[info] Hooks path set to .hooky directory");

    fs::create_dir_all(pre_commit_dir).expect("Failed to create .hooky directory");

    if !no_pre_commit {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o755)
            .open(pre_commit_dir.join("pre-commit"))
            .expect("Failed to create pre-commit file");

        file.write_all(b"#!/usr/bin/env sh\n# Run pre-commit hooks\n\nexit 0")
            .expect("Failed to write to pre-commit file");

        println!("[info] Created pre-commit file");
    }
}

fn uninstall() {
    let pre_commit_dir = path::Path::new(".hooky");

    if !path::Path::new(".git").exists() {
        println!("[error] git cannot be found");

        return;
    }

    if !pre_commit_dir.exists() {
        println!("[info] hooky is not installed");

        return;
    }

    Command::new("git")
        .arg("config")
        .arg("--unset")
        .arg("core.hooksPath")
        .spawn()
        .expect("Failed to unset hooks path");

    fs::remove_dir_all(pre_commit_dir).expect("Failed to remove .hooky directory");

    println!("[info] Uninstalled hooky");
}

fn add_hook(hook: &str) {
    if !ALLOWED_HOOKS.contains(&hook) {
        println!("[error] Hook not allowed");

        return;
    }

    let pre_commit_dir = path::Path::new(".hooky");

    if !pre_commit_dir.exists() {
        println!("[error] .hooky directory not found");
        println!("[hint] try running `hooky init` first");

        return;
    }

    if !path::Path::new(".git").exists() {
        println!("[error] git cannot be found");

        return;
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o755)
        .open(pre_commit_dir.join(hook))
        .expect("Failed to create hook file");

    let hook_content = format!("#!/usr/bin/env sh\n# Run {} hook\n\nexit 0", hook);
    file.write_all(hook_content.as_bytes())
        .expect("Failed to write to hook file");

    println!("[info] Created hook file");
}

fn help() {
    println!("hooky init - Initialize hooky. Optionally pass --no-pre-commit to skip creating a pre-commit hook");
    println!("hooky uninstall - Uninstall hooky");
    println!("hooky add <hook> - Add a hook");
}

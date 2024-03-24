use std::fs;
use std::io::prelude::*;
use std::path;
use std::process::Command;

#[cfg(target_family = "unix")]
use std::os::unix::prelude::OpenOptionsExt;

#[cfg(target_family = "windows")]
use std::os::windows::fs::OpenOptionsExt;

pub const ALLOWED_HOOKS: [&str; 13] = [
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

pub fn init(no_pre_commit: bool) {
    let pre_commit_dir = path::Path::new(".hooky");

    if !path::Path::new(".git").exists() {
        println!("[error] git cannot be found");

        return;
    }

    if pre_commit_dir.exists() {
        println!("[info] hooky is already installed");

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
        let mut file = create_file(pre_commit_dir.join("pre-commit"));

        file.write_all(b"#!/usr/bin/env sh\n# Run pre-commit hooks\n\nexit 0")
            .expect("Failed to write to pre-commit file");

        println!("[info] Created pre-commit file");
    }
}

pub fn uninstall() {
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

pub fn add_hook(hook: &str) {
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
    let mut file = create_file(pre_commit_dir.join(hook));

    let hook_content = format!("#!/usr/bin/env sh\n# Run {} hook\n\nexit 0", hook);
    file.write_all(hook_content.as_bytes())
        .expect("Failed to write to hook file");

    println!("[info] Created hook file");
}

fn create_file<P>(path: P) -> fs::File
where
    P: AsRef<path::Path>,
{
    let mut options = fs::OpenOptions::new();

    options.create(true).write(true);

    #[cfg(target_family = "windows")]
    {
        options.access_mode(0o755);
    }

    #[cfg(target_family = "unix")]
    {
        options.mode(0o755);
    }

    options.open(path).expect("Failed to create file")
}

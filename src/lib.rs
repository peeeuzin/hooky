use std::fs;
use std::io::prelude::*;
use std::path;
use std::process::Command;

mod logs;
pub use logs::Logs;

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

pub fn initialize_build() {
    let hooky = Hooky::new(true);
    hooky.initialize(true);
}

pub struct Hooky {
    logs: Logs,
}

impl Hooky {
    pub fn new(quiet_mode: bool) -> Self {
        let logs = Logs { quiet_mode };

        Self { logs }
    }

    pub fn initialize(&self, no_pre_commit: bool) {
        let pre_commit_dir = path::Path::new(".hooky");

        if !path::Path::new(".git").exists() {
            self.logs.error("git cannot be found");

            return;
        }

        if !pre_commit_dir.exists() {
            fs::create_dir_all(pre_commit_dir).expect("Failed to create .hooky directory");

            if !no_pre_commit {
                self.add_hook("pre-commit")
            }
        }

        Command::new("git")
            .arg("config")
            .arg("core.hooksPath")
            .arg(pre_commit_dir)
            .spawn()
            .expect("Failed to set hooks path");

        self.logs.ok("hooks path set to .hooky directory");
    }

    pub fn uninstall(&self) {
        let pre_commit_dir = path::Path::new(".hooky");

        if !path::Path::new(".git").exists() {
            self.logs.error("git cannot be found");

            return;
        }

        if !pre_commit_dir.exists() {
            self.logs.error("hooky is not installed");

            return;
        }

        Command::new("git")
            .arg("config")
            .arg("--unset")
            .arg("core.hooksPath")
            .spawn()
            .expect("Failed to unset hooks path");

        fs::remove_dir_all(pre_commit_dir).expect("Failed to remove .hooky directory");

        self.logs.ok("uninstalled hooky");
    }

    pub fn add_hook(&self, hook: &str) {
        if !ALLOWED_HOOKS.contains(&hook) {
            self.logs.error("hook not allowed");

            return;
        }

        let pre_commit_dir = path::Path::new(".hooky");

        if !pre_commit_dir.exists() {
            self.logs.error(".hooky directory not found");
            self.logs.info("try running `hooky init` first");

            return;
        }

        if !path::Path::new(".git").exists() {
            self.logs.error("git cannot be found");

            return;
        }
        let mut file = create_file(pre_commit_dir.join(hook));

        let hook_content = hook_file_placeholder(hook);
        file.write_all(hook_content.as_bytes())
            .expect("Failed to write to hook file");

        self.logs.ok("created hook file");
    }
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

fn hook_file_placeholder(hook: &str) -> String {
    format!("#!/usr/bin/env sh\n# Run {} hook\n\nexit 0", hook)
}

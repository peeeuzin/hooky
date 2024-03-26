pub struct Logs {
    pub quiet_mode: bool,
}

impl Logs {
    pub fn error(&self, message: &str) {
        if !self.quiet_mode {
            eprintln!("\x1b[31m[Error]\x1b[0m {}", message);
        }
    }

    pub fn info(&self, message: &str) {
        if !self.quiet_mode {
            println!("[Info] {}", message);
        }
    }

    pub fn ok(&self, message: &str) {
        if !self.quiet_mode {
            println!("\x1b[32m[Ok]\x1b[0m {}", message);
        }
    }
}

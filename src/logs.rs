pub struct Logs;

impl Logs {
    pub fn error(message: &str) {
        eprintln!("\x1b[31m[Error]\x1b[0m {}", message);
    }

    pub fn info(message: &str) {
        println!("[Info] {}", message);
    }

    pub fn ok(message: &str) {
        println!("\x1b[32m[Ok]\x1b[0m {}", message);
    }
}

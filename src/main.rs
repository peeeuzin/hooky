use std::env;

fn main() {
    println!("Hooky - v{}", env!("CARGO_PKG_VERSION"));
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
        "init" => hooky::init(args.contains(&"--no-pre-commit".to_string())),
        "uninstall" => hooky::uninstall(),
        "add" => hooky::add_hook(&args[0]),

        _ => println!("Command not found"),
    }
}

fn help() {
    println!("hooky init - Initialize hooky. Optionally pass --no-pre-commit to skip creating a pre-commit hook");
    println!("hooky uninstall - Uninstall hooky");
    println!("hooky add <hook> - Add a hook");
}

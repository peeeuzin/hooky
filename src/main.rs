use std::env;

use hooky::Hooky;

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
    let use_quiet = args.contains(&"--quiet".to_string());
    let hooky = Hooky::new(use_quiet);

    if !use_quiet {
        println!("Hooky - v{}", env!("CARGO_PKG_VERSION"));
        println!();
    }

    match command {
        "init" => hooky.initialize(args.contains(&"--no-pre-commit".to_string())),
        "uninstall" => hooky.uninstall(),
        "add" => hooky.add_hook(&args[0]),

        _ => help(),
    }
}

fn help() {
    println!("Usage: hooky <command> [args]");
    println!();
    println!("hooky init - Initialize hooky. Optionally pass --no-pre-commit to skip creating a pre-commit hook");
    println!("hooky uninstall - Uninstall hooky");
    println!("hooky add <hook> - Add a hook");
    println!();
    println!("Options:");
    println!("--quiet - Run hooky in quiet mode");
}

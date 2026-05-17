use landlock::{
    ABI, Access, AccessFs, PathBeneath, PathFd, Ruleset, RulesetAttr, RulesetCreatedAttr,
};
use clap::{Parser, Subcommand};
use std::process::Command;
use std::io;
use std::io::Write;


/// lnx-runner
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// define available commands
#[derive(Subcommand)]
enum Commands {
    Run { path: String },
    Install { path: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cli = Cli::parse();

    // Roll through commands
    match cli.command {Commands::Run { path } => {

        // ABI, permissions and other landlock related stuff will be defined here
        let abi = ABI::V1;
        let protected_actions = AccessFs::from_all(abi);

        // These permissions will be given to the sub-processes as well
        let permissions = AccessFs::ReadFile | AccessFs::Execute | AccessFs::ReadDir;

        // Define a ruleset and apply permissions
        let mut ruleset = Ruleset::default()
        .handle_access(protected_actions)?
        .create()?;

        // Define the rules which the landlock process has access to
        let path_fd = PathFd::new(&path)?;
        ruleset = ruleset.add_rule(PathBeneath::new(path_fd, permissions))?;

        print!("Please specify the dictionaries where this script has access to:");

        // Flush stdout to make sure prompt actually prints before before waiting for input
        io::stdout().flush().unwrap();

        // Create a new, empty string
        let mut input = String::new();

        // Read user string
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Create a new vector permission array, seperated by a single whitespace
        let permissions_list: Vec::<String> = input
            .split_whitespace()
            .map(|s| s.trim().to_string())
            .collect();

        // Add the system paths
        // TODO: Don't forget to clean this spaghetti up once you get it working properly
        for p in &permissions_list {
            if let Ok(p_fd) = PathFd::new(p) {
                ruleset = ruleset.add_rule(PathBeneath::new(p_fd, permissions))?;
            }
        }

        println!("The script has access to...: {:?}", permissions_list);

        // Now restrict processes using the ruleset, remember that starting from this point you are bound by these rules as well
        ruleset.restrict_self()?;

        // Debugging
        println!("Sandbox active. Executing...");
        let status = Command::new("sh").arg(&path).status()?;

        if status.success() {
            println!("lnx-runner worked without errors!")
        } else {
            match status.code() {
                Some(code) => eprintln!("Script exited with code: {}", code),
                None => eprintln!("Script is terminated"),
            }
        }
    }

    // TODO: Implement installation and name-scapes for actual sand-boxing outside of restrictions
    Commands::Install { path } => {
            println!("Installing to: {}", path);
        }
    }

    Ok(())
}

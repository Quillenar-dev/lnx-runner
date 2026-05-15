use landlock::{
    ABI, Access, AccessFs, PathBeneath, PathFd, Ruleset, RulesetAttr, RulesetCreatedAttr,
};
use clap::{Parser, Subcommand};
use std::process::Command;


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

        let path_fd = PathFd::new(&path)?;
        ruleset = ruleset.add_rule(PathBeneath::new(path_fd, permissions))?;

        // Add the system paths
        // TODO: Don't forget to clean this spaghetti up once you get it working properly
        let permitted_paths = ["/usr", "/lib", "/lib64", "/etc", "/dev/null", "/dev/tty", "."];
        for p in permitted_paths {
            if let Ok(p_fd) = PathFd::new(p) {
                ruleset = ruleset.add_rule(PathBeneath::new(p_fd, permissions))?;
            }
        }

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

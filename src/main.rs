use landlock::{
    ABI, Access, AccessFs, PathBeneath, PathFd, Ruleset, RulesetAttr, RulesetCreatedAttr,
};
use clap::{Parser, Subcommand};

/// landlock implementation
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { path: String },
    Install { path: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // 1. We match on the command itself
    match cli.command {
        // 2. If the command is 'Run', we extract the 'path' string
        Commands::Run { path } => {
            println!("The provided path is: {}", path);

            let protected_actions = AccessFs::from_all(ABI::V1);

            let ruleset = Ruleset::default()
            .handle_access(protected_actions)?
            .create()?;

            // 3. We use the 'path' variable we just extracted
            let path_fd = PathFd::new(&path)?;
            let ruleset = ruleset.add_rule(PathBeneath::new(path_fd, AccessFs::ReadFile))?;

            ruleset.restrict_self()?;

            // Do the actual work here while restricted!
            println!("Sandbox active. Reading from {} is allowed.", path);
        }
        // 4. We MUST handle the 'Install' case too, or the code won't compile
        Commands::Install { path } => {
            println!("Installing to: {}", path);
        }
    }

    Ok(())
}

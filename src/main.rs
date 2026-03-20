use clap::{Parser, Subcommand};
use colored::Colorize;
use yourmom::cli;

#[derive(Parser)]
#[command(name = "yourmom")]
#[command(about = "VQ — Version 3: Quantum. The most cursed fucking language in existence.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile .yourmom files (daddy knocked up the source)
    #[command(alias = "build")]
    Childmake {
        /// Path to .yourmom file or .yourdad project
        #[arg(value_name = "FILE")]
        input: String,

        /// Output binary name
        #[arg(short, long)]
        output: Option<String>,

        /// Link extra C libraries (e.g. -l raylib -l pthread)
        #[arg(short = 'l', long = "lib", value_name = "LIB")]
        libs: Vec<String>,
    },

    /// Clean build artifacts (abort the build)
    Abortion {
        /// Also remove quantum runtime cache and generated .yourmom.c files
        #[arg(long)]
        deep: bool,
    },

    /// Show dependency tree (family tree)
    #[command(name = "family-tree")]
    FamilyTree {
        /// Path to .yourdad file
        project: String,
    },

    /// Remove binary (divorce from executable)
    Divorce {
        /// Binary to remove
        binary: String,
    },

    /// Check for unused dependencies (deadbeat check)
    Deadbeat {
        /// Path to .yourdad file
        project: String,
    },

    /// Manage .momjoke imports (custody battle)
    Custody {
        /// List, add, or remove .momjoke files
        #[command(subcommand)]
        action: CustodyAction,
    },

    /// Compile and run immediately
    Run {
        /// .yourmom file to compile and run
        file: String,

        /// Link extra C libraries (e.g. -l raylib)
        #[arg(short = 'l', long = "lib", value_name = "LIB")]
        libs: Vec<String>,
    },

    /// Start REPL (TODO: future feature)
    Repl,
}

#[derive(Subcommand)]
enum CustodyAction {
    /// List loaded .momjoke files
    List,
    /// Add a .momjoke dependency
    Add { file: String },
    /// Remove a .momjoke dependency
    Remove { file: String },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Childmake { input, output, libs } => {
            cli::childmake(&input, output.as_deref(), &libs)
        }
        Commands::Abortion { deep } => {
            cli::abortion(deep)
        }
        Commands::FamilyTree { project } => {
            cli::family_tree(&project)
        }
        Commands::Divorce { binary } => {
            cli::divorce(&binary)
        }
        Commands::Deadbeat { project } => {
            cli::deadbeat(&project)
        }
        Commands::Custody { action } => {
            match action {
                CustodyAction::List => cli::custody_list(),
                CustodyAction::Add { file } => cli::custody_add(&file),
                CustodyAction::Remove { file } => cli::custody_remove(&file),
            }
        }
        Commands::Run { file, libs } => {
            cli::run(&file, &libs)
        }
        Commands::Repl => {
            println!("{}", "REPL not implemented yet (coming soon, you impatient bastard!)".yellow());
            Ok(())
        }
    };

    if let Err(e) = result {
        match &e {
            yourmom::YourMomError::Diagnostic(diag) => {
                diag.print();
            }
            yourmom::YourMomError::FileNotFound(path) => {
                eprintln!("{} {}", "💀".red(), e.to_string().red());
                eprintln!("   {}", format!("Hint: Check if '{}' exists in this directory", path).yellow());
            }
            yourmom::YourMomError::GccFailed => {
                eprintln!("{} {}", "💀".red(), e.to_string().red());
                eprintln!("   {}", "Hint: Check output.c for C compilation errors. Is gcc installed?".yellow());
                eprintln!("   {}", "Hint: If using raylib etc, make sure -l flags are correct".yellow());
            }
            yourmom::YourMomError::LexerError { line, column, message } => {
                eprintln!("{} Lexer Error at {}:{} — {}", "💀".red(), line, column, message.red());
            }
            _ => {
                eprintln!("{} {}", "💀".red(), e.to_string().as_str().red());
            }
        }
        std::process::exit(1);
    }
}

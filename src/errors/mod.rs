use thiserror::Error;
use colored::Colorize;
use rand::seq::SliceRandom;

pub type Result<T> = std::result::Result<T, YourMomError>;

// ── Diagnostic infrastructure ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct Diagnostic {
    pub severity: Severity,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub joke: String,
    pub help: Option<String>,
    pub note: Option<String>,
    pub source: String,
}

impl Diagnostic {
    pub fn print(&self) {
        let severity_str = match self.severity {
            Severity::Error   => "ERROR".red().bold(),
            Severity::Warning => "WARNING".yellow().bold(),
        };

        // Header
        eprintln!("{} {}: {}",
            "💀".bright_red(),
            severity_str,
            self.message.bright_white()
        );

        // Location
        eprintln!("  {} {}:{}:{}",
            "┌─".blue(),
            self.file,
            self.line,
            self.column
        );
        eprintln!("  {}", "│".blue());

        // Source context
        let lines: Vec<&str> = self.source.lines().collect();
        let line_idx = self.line.saturating_sub(1);

        if line_idx > 0 && line_idx <= lines.len() {
            eprintln!("{:>3} {} {}",
                (line_idx).to_string().blue(),
                "│".blue(),
                lines[line_idx - 1].dimmed()
            );
        }

        if line_idx < lines.len() {
            eprintln!("{:>3} {} {}",
                (line_idx + 1).to_string().blue().bold(),
                "│".blue(),
                lines[line_idx]
            );

            // Caret pointing at column
            let col = self.column.saturating_sub(1);
            eprintln!("    {} {}{}",
                "│".blue(),
                " ".repeat(col),
                "^".red().bold()
            );
        }

        // Joke
        eprintln!("    {} {}",
            "│".blue(),
            self.joke.yellow()
        );
        eprintln!("  {}", "│".blue());

        // Help
        if let Some(help) = &self.help {
            eprintln!("  {} {}", "= help:".cyan().bold(), help);
        }

        // Note
        if let Some(note) = &self.note {
            eprintln!("  {} {}", "= note:".blue().bold(), note);
        }

        eprintln!();
    }
}

// ── Error enum ────────────────────────────────────────────────────────────────

#[derive(Error, Debug)]
pub enum YourMomError {
    #[error("💀 Lexer error at line {line}, column {column} — {message}")]
    LexerError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Holy shit — yo mama so ugly, the parser refused to look at line {0}")]
    ParseError(usize),

    #[error("Are you fucking kidding me — yo mama so dumb, she {0}")]
    SyntaxError(String),

    #[error("Yo mama so broke, GCC refused to compile this crap")]
    GccFailed,

    #[error("Goddamn it — yo mama so lost, she can't find the fucking file: {0}")]
    FileNotFound(String),

    /// Rich diagnostic with source context
    #[error("compile error — see above for details")]
    Diagnostic(Box<Diagnostic>),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}


// ── Joke database ─────────────────────────────────────────────────────────────

pub struct JokeDatabase {
    compile_errors: Vec<&'static str>,
    runtime_errors: Vec<&'static str>,
}

impl JokeDatabase {
    pub fn new() -> Self {
        Self {
            compile_errors: vec![
                "forgot a fucking semicolon — wait, we don't use those, you dumb bastard",
                "tried to use a damn variable before declaring it",
                "thought 'yo_mama_so_fat' was a type — what the shit",
                "used 'daddy_left' but that bastard never came back",
                "expected a quantum collapse but got family drama instead, holy hell",
                "wrote code so bad it made the compiler cry, you absolute ass",
                "tried to divide by zero like a goddamn idiot",
            ],
            runtime_errors: vec![
                "Yo mama so fat, her ass caused a goddamn stack overflow",
                "Yo mama so dumb, she dereferenced a NULL pointer — what the fuck",
                "Yo mama so ugly, even the CPU refused to execute that shit",
                "Yo mama so slow, she caused heap corruption, the dumb bitch",
                "Yo mama so cheap, she freed memory twice like a damn bastard",
                "Yo mama so fat, she exceeded the fucking address space",
                "Yo mama so random, she corrupted the RNG seed — holy shitballs",
                "Yo mama so nasty, she segfaulted before she even started, crap",
                "Yo mama so dumb, she wrote to a read-only page, fucking hell",
                "Yo mama so wide, she overflowed the goddamn heap",
            ],
        }
    }

    pub fn random_compile_error(&self) -> String {
        let joke = self.compile_errors
            .choose(&mut rand::thread_rng())
            .unwrap();
        format!("Are you fucking kidding me — yo mama so dumb, she {}", joke)
    }

    pub fn random_runtime_error(&self) -> &str {
        self.runtime_errors
            .choose(&mut rand::thread_rng())
            .unwrap()
    }
}

impl Default for JokeDatabase {
    fn default() -> Self {
        Self::new()
    }
}

// ── Context-aware jokes ───────────────────────────────────────────────────────

pub fn joke_for_unexpected_token(expected: &str, got: &str) -> String {
    format!("Yo mama so dumb, she handed me a {} when I needed a {}", got, expected)
}

pub fn joke_for_unclosed_brace() -> String {
    "Yo mama so forgetful, she left a brace wide open".to_string()
}

pub fn joke_for_bad_function_name(got: &str) -> String {
    format!("Yo mama named her function '{}' — what the fuck kind of name is that", got)
}

pub fn joke_for_unterminated_string() -> String {
    "Yo mama so wordy, she left a string without a closing quote".to_string()
}

// ── Legacy helper ─────────────────────────────────────────────────────────────

pub fn report_error(error: &YourMomError) {
    match error {
        YourMomError::Diagnostic(diag) => diag.print(),
        _ => eprintln!("{} {}", "💀 OH FUCK, FAMILY DRAMA:".red().bold(), error.to_string().red()),
    }
}

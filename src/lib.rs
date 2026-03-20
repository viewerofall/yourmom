pub mod lexer;
pub mod parser;
pub mod codegen;
pub mod errors;
pub mod build_system;
pub mod cli;
pub mod runtime;

pub use errors::{YourMomError, Result};
pub use lexer::{Lexer, Token, TokenKind};
pub use parser::{Parser, Ast};
pub use codegen::Codegen;

/// Compile a single .yourmom source to C code string.
pub fn compile_to_c(source: &str, shortcuts: &lexer::ShortcutMap) -> Result<String> {
    compile_to_c_named(source, shortcuts, "<input>")
}

/// Compile to C with a filename for error reporting.
pub fn compile_to_c_named(source: &str, shortcuts: &lexer::ShortcutMap, filename: &str) -> Result<String> {
    let mut lex = Lexer::with_filename(source, shortcuts, filename);
    let tokens = lex.tokenize()?;
    let mut parser = Parser::with_context(tokens, filename.to_string(), source.to_string());
    let ast = parser.parse()?;
    let mut codegen = Codegen::new();
    Ok(codegen.generate(&ast)?)
}

/// Compile .yourmom source to a native binary.
///
/// - `extra_libs`: lib names explicitly passed by the user (e.g. from -l flags or .yourdad)
/// - Auto-detects additional libraries from the generated C code and links them automatically.
/// - Uses pkg-config when available for correct include paths and link flags.
pub fn compile_to_binary_named(
    source: &str,
    output: &str,
    shortcuts: &lexer::ShortcutMap,
    filename: &str,
    extra_libs: &[String],
) -> Result<()> {
    let c_code = compile_to_c_named(source, shortcuts, filename)?;
    std::fs::write("output.c", &c_code)?;

    run_gcc("output.c", output, &c_code, extra_libs)
}

/// Run GCC on a C file, auto-detecting libraries from the C source.
///
/// `c_code` is the C source text to scan for library patterns.
/// `extra_libs` are manually declared lib names (plain names, no -l prefix).
pub fn run_gcc(
    c_file: &str,
    output: &str,
    c_code: &str,
    extra_libs: &[String],
) -> Result<()> {
    use build_system::{detect_libs_from_c, print_detected_libs};

    // Auto-detect libs from generated C, skipping ones already declared
    let detected = detect_libs_from_c(c_code, extra_libs);
    print_detected_libs(&detected);

    // Inject #include directives for detected libraries at the top of the C file.
    // We prepend them after the existing content so the runtime header stays first.
    let mut include_block = String::new();
    for lib in &detected {
        for header in &lib.headers {
            let line = format!("#include <{}>\n", header);
            // Only add if not already present in the file
            if !c_code.contains(&line) {
                include_block.push_str(&line);
            }
        }
    }
    if !include_block.is_empty() {
        // Insert after the first #include line (the runtime header)
        let injected = if let Some(pos) = c_code.find('\n') {
            format!("{}\n{}{}", &c_code[..pos], include_block, &c_code[pos + 1..])
        } else {
            format!("{}\n{}", c_code, include_block)
        };
        std::fs::write(c_file, &injected)?;
    }

    // Build the GCC command
    let mut args: Vec<String> = vec![
        "-I.".to_string(),
        c_file.to_string(),
        "-o".to_string(),
        output.to_string(),
    ];

    // Include paths from pkg-config (for auto-detected libs)
    for lib in &detected {
        args.extend(lib.cflags.clone());
    }

    // Always link math
    args.push("-lm".to_string());

    // Explicitly declared extra libs
    for lib in extra_libs {
        args.push(format!("-l{}", lib));
    }

    // Auto-detected lib link flags
    for lib in &detected {
        args.extend(lib.ldflags.clone());
    }

    let status = std::process::Command::new("gcc")
        .args(&args)
        .status()?;

    if !status.success() {
        return Err(errors::YourMomError::GccFailed);
    }

    Ok(())
}

/// Convenience wrapper with no extra libs (backwards compat).
pub fn compile_to_binary(
    source: &str,
    output: &str,
    shortcuts: &lexer::ShortcutMap,
) -> Result<()> {
    compile_to_binary_named(source, output, shortcuts, "<input>", &[])
}

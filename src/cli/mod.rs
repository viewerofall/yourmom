use colored::Colorize;
use crate::errors::{Result, YourMomError};
use crate::lexer::ShortcutMap;
use crate::build_system::YourDadConfig;
use std::path::Path;

pub fn childmake(input: &str, output: Option<&str>, extra_libs: &[String]) -> Result<()> {
    println!("{}", "🤰 Hell yeah, mama got pregnant (compilation started)...".yellow());

    if input.ends_with(".yourdad") || input.ends_with(".dad") {
        return childmake_project(input);
    }

    let output_name = output.unwrap_or("yourmom_program");
    let shortcuts = ShortcutMap::default_stdlib();

    let source = std::fs::read_to_string(input)
        .map_err(|_| YourMomError::FileNotFound(input.to_string()))?;

    crate::runtime::write_runtime()?;
    crate::compile_to_binary_named(&source, output_name, &shortcuts, input, extra_libs)?;

    println!("{}", format!("👶 Fuck yeah! Birth successful — binary: {}", output_name).green().bold());
    Ok(())
}

fn childmake_project(dad_file: &str) -> Result<()> {
    println!("{}", format!("📋 Reading project config: {}", dad_file).cyan());

    let config = YourDadConfig::parse(dad_file)?;

    // Build shortcut map: start with stdlib, merge any project .momjoke files
    let mut shortcuts = ShortcutMap::default_stdlib();
    for jk_file in &config.momjoke_files {
        let path = jk_file.as_str();
        if Path::new(path).exists() {
            let extra = ShortcutMap::from_file(path)?;
            shortcuts.merge(extra);
            println!("{}", format!("  📦 Loaded shortcuts: {}", jk_file).cyan());
        } else {
            println!("{}", format!("  ⚠️  Warning: momjoke file not found: {}", jk_file).yellow());
        }
    }

    crate::runtime::write_runtime()?;

    // Compile all source files to C, collecting generated code for lib detection
    let mut c_files: Vec<String> = Vec::new();
    let mut all_c_code = String::new();

    for src in &config.sources {
        if !Path::new(src).exists() {
            return Err(YourMomError::FileNotFound(src.clone()));
        }

        println!("{}", format!("  🔨 Compiling: {}", src).cyan());
        let source = std::fs::read_to_string(src)
            .map_err(|_| YourMomError::FileNotFound(src.clone()))?;

        let c_code = crate::compile_to_c_named(&source, &shortcuts, src)?;
        all_c_code.push_str(&c_code);

        let c_out = format!("{}.c", src);
        std::fs::write(&c_out, &c_code)?;
        c_files.push(c_out);
    }

    // Auto-detect libraries from the combined generated C
    use crate::build_system::{detect_libs_from_c, print_detected_libs};
    let detected = detect_libs_from_c(&all_c_code, &config.c_libs);
    print_detected_libs(&detected);

    // Inject #include directives for detected libs into each generated C file
    if !detected.is_empty() {
        let mut include_block = String::new();
        for lib in &detected {
            for header in &lib.headers {
                let line = format!("#include <{}>\n", header);
                if !all_c_code.contains(&line) {
                    include_block.push_str(&line);
                }
            }
        }
        if !include_block.is_empty() {
            for c_file in &c_files {
                if let Ok(existing) = std::fs::read_to_string(c_file) {
                    let injected = if let Some(pos) = existing.find('\n') {
                        format!("{}\n{}{}", &existing[..pos], include_block, &existing[pos + 1..])
                    } else {
                        format!("{}\n{}", existing, include_block)
                    };
                    let _ = std::fs::write(c_file, injected);
                }
            }
        }
    }

    // Build GCC command: base flags + all c files + -lm + declared libs + auto-detected
    let mut gcc_args: Vec<String> = config.gcc_flags();

    // Include paths from pkg-config for auto-detected libs
    for lib in &detected {
        gcc_args.extend(lib.cflags.clone());
    }

    // Source files
    gcc_args.extend(c_files.clone());

    gcc_args.push("-lm".to_string());

    // Auto-detected link flags
    for lib in &detected {
        gcc_args.extend(lib.ldflags.clone());
    }

    gcc_args.extend(["-o".to_string(), config.output.clone()]);

    println!("{}", format!("  🔗 Linking: {}", config.output).cyan());
    let status = std::process::Command::new("gcc")
        .args(&gcc_args)
        .status()?;

    if !status.success() {
        return Err(YourMomError::GccFailed);
    }

    println!("{}", format!("👶 Fuck yeah! Birth successful — binary: {}", config.output).green().bold());
    Ok(())
}

pub fn abortion(deep: bool) -> Result<()> {
    println!("{}", "🗑️  Aborting that shit — cleaning build artifacts...".yellow());

    let _ = std::fs::remove_file("output.c");
    let _ = std::fs::remove_file("yourmom_program");
    let _ = std::fs::remove_file("a.out");
    let _ = std::fs::remove_file("quantum_runtime.h");

    if deep {
        println!("{}", "Deep cleaning all quantum crap too...".yellow());
        // Remove any .c files from .yourmom compilation
        for entry in std::fs::read_dir(".").unwrap_or_else(|_| panic!("can't read dir")) {
            if let Ok(e) = entry {
                let name = e.file_name().to_string_lossy().to_string();
                if name.ends_with(".yourmom.c") || name == "output.c" {
                    let _ = std::fs::remove_file(e.path());
                }
            }
        }
    }

    println!("{}", "✅ Abortion complete — that shit is gone".green());
    Ok(())
}

pub fn family_tree(project: &str) -> Result<()> {
    println!("{}", format!("👨‍👩‍👧‍👦 Family tree for: {}", project).cyan().bold());

    let config = YourDadConfig::parse(project)?;
    let tree = config.format_tree();

    for line in tree.lines() {
        println!("  {}", line);
    }

    println!();
    println!("{}", format!("  optimization: -O{}", config.optimization).dimmed());
    if config.debug {
        println!("{}", "  debug: enabled (-g)".dimmed());
    }

    Ok(())
}

pub fn divorce(binary: &str) -> Result<()> {
    println!("{}", format!("💔 Divorcing from that bastard binary: {}", binary).yellow());

    std::fs::remove_file(binary)
        .map_err(|_| YourMomError::FileNotFound(binary.to_string()))?;

    println!("{}", "✅ Divorce finalized — good fucking riddance".green());
    Ok(())
}

pub fn deadbeat(project: &str) -> Result<()> {
    println!("{}", "🔍 Checking for deadbeat dependencies, those lazy asses...".yellow());

    let config = YourDadConfig::parse(project)?;

    // Collect all text from .yourmom source files
    let mut all_source = String::new();
    for src in &config.sources {
        if let Ok(content) = std::fs::read_to_string(src) {
            all_source.push_str(&content);
        }
    }

    let mut deadbeats: Vec<&String> = Vec::new();
    for lib in &config.c_libs {
        // Check if the lib name appears anywhere in the source (as a function call or comment)
        if !all_source.contains(lib.as_str()) {
            deadbeats.push(lib);
        }
    }

    if deadbeats.is_empty() {
        println!("{}", "✅ No deadbeats found — every lib is pulling its weight".green());
    } else {
        println!("{}", format!("💀 Found {} deadbeat lib(s):", deadbeats.len()).red());
        for lib in &deadbeats {
            println!("  {} -l{} (not referenced in source)", "❌".red(), lib);
        }
        println!("{}", "  Hint: Remove unused libs from daddy_says_use_protection {}".yellow());
    }

    Ok(())
}

pub fn custody_list() -> Result<()> {
    println!("{}", "📋 .momjoke files in custody:".cyan());

    let mut found = false;

    // Check current directory for .momjoke files
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".momjoke") {
                println!("  📦 {}", name);
                found = true;
            }
        }
    }

    // Check ~/.yourmom/momjoke/ if it exists
    if let Some(home) = std::env::var("HOME").ok() {
        let global_dir = format!("{}/.yourmom/momjoke", home);
        if let Ok(entries) = std::fs::read_dir(&global_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".momjoke") {
                    println!("  🌍 {} (global)", name);
                    found = true;
                }
            }
        }
    }

    if !found {
        println!("  (none found — you're in sole custody of nothing, you deadbeat)");
    }

    Ok(())
}

pub fn custody_add(file: &str) -> Result<()> {
    if !file.ends_with(".momjoke") {
        return Err(YourMomError::SyntaxError(
            format!("'{}' doesn't end in .momjoke — only momjoke files allowed in custody", file)
        ));
    }

    if !Path::new(file).exists() {
        return Err(YourMomError::FileNotFound(file.to_string()));
    }

    // Copy to project dir (current dir) if not already there
    let dest_name = Path::new(file)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    if dest_name != file {
        std::fs::copy(file, &dest_name)?;
        println!("{}", format!("➕ Copied {} → ./{}", file, dest_name).green());
    } else {
        println!("{}", format!("➕ {} is already in the project directory", file).green());
    }

    println!("{}", "  Remember to add it to daddy_says_wear_these_hats {} in your .yourdad".yellow());
    Ok(())
}

pub fn custody_remove(file: &str) -> Result<()> {
    let path = Path::new(file);
    if !path.exists() {
        return Err(YourMomError::FileNotFound(file.to_string()));
    }

    std::fs::remove_file(file)?;
    println!("{}", format!("➖ Removed {} — bye bitch", file).yellow());
    Ok(())
}

pub fn run(file: &str, extra_libs: &[String]) -> Result<()> {
    println!("{}", "🏃 Compiling and running that shit...".yellow());

    childmake(file, Some("temp_yourmom_binary"), extra_libs)?;

    let status = std::process::Command::new("./temp_yourmom_binary")
        .status()?;

    let _ = std::fs::remove_file("temp_yourmom_binary");

    if !status.success() {
        println!("{}", "💀 Oh fuck — runtime family drama occurred".red().bold());
    }

    Ok(())
}

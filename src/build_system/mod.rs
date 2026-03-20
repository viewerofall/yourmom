use std::fs;
use colored::Colorize;
use crate::errors::{Result, YourMomError};

// ── Library auto-detection table ──────────────────────────────────────────────
//
// Each entry: (function/symbol patterns to scan for, pkg-config name, fallback -l name)
// We scan the *generated C code* for these patterns to decide what to link.

struct LibSig {
    patterns:  &'static [&'static str],
    pkgconfig: &'static str,   // name for pkg-config lookup
    fallback:  &'static str,   // plain -l name if pkg-config unavailable
    headers:   &'static [&'static str], // C headers to auto-include
}

static KNOWN_LIBS: &[LibSig] = &[
    LibSig {
        patterns:  &["InitWindow", "BeginDrawing", "EndDrawing", "CloseWindow",
                     "DrawText", "DrawRectangle", "DrawCircle", "DrawTexture",
                     "LoadTexture", "BeginMode2D", "BeginMode3D", "BeginShaderMode",
                     "CheckCollisionRecs", "GetMousePosition", "IsKeyPressed",
                     "IsMouseButtonPressed", "WindowShouldClose", "SetTargetFPS",
                     "ClearBackground", "GetTime", "GetScreenWidth", "GetScreenHeight"],
        pkgconfig: "raylib",
        fallback:  "raylib",
        headers:   &["raylib.h"],
    },
    LibSig {
        patterns:  &["SDL_Init", "SDL_CreateWindow", "SDL_CreateRenderer",
                     "SDL_RenderPresent", "SDL_Delay", "SDL_Quit", "SDL_PollEvent",
                     "SDL_RenderClear", "SDL_DestroyWindow", "SDL_DestroyRenderer"],
        pkgconfig: "sdl2",
        fallback:  "SDL2",
        headers:   &["SDL2/SDL.h"],
    },
    LibSig {
        patterns:  &["Mix_OpenAudio", "Mix_LoadMUS", "Mix_PlayMusic",
                     "Mix_CloseAudio", "Mix_LoadWAV", "Mix_PlayChannel"],
        pkgconfig: "SDL2_mixer",
        fallback:  "SDL2_mixer",
        headers:   &["SDL2/SDL_mixer.h"],
    },
    LibSig {
        patterns:  &["IMG_Init", "IMG_Load", "IMG_LoadTexture", "IMG_Quit"],
        pkgconfig: "SDL2_image",
        fallback:  "SDL2_image",
        headers:   &["SDL2/SDL_image.h"],
    },
    LibSig {
        patterns:  &["TTF_Init", "TTF_OpenFont", "TTF_RenderText", "TTF_Quit"],
        pkgconfig: "SDL2_ttf",
        fallback:  "SDL2_ttf",
        headers:   &["SDL2/SDL_ttf.h"],
    },
    LibSig {
        patterns:  &["glfwInit", "glfwCreateWindow", "glfwMakeContextCurrent",
                     "glfwSwapBuffers", "glfwPollEvents", "glfwTerminate",
                     "glfwWindowShouldClose", "glfwGetKey"],
        pkgconfig: "glfw3",
        fallback:  "glfw",
        headers:   &["GLFW/glfw3.h"],
    },
    LibSig {
        patterns:  &["glBegin", "glEnd", "glViewport", "glClear", "glClearColor",
                     "glGenTextures", "glBindTexture", "glVertex3f", "glMatrixMode",
                     "glGenBuffers", "glBindBuffer", "glBufferData", "glDrawArrays",
                     "glEnableVertexAttribArray", "glVertexAttribPointer",
                     "glUseProgram", "glCreateShader", "glShaderSource"],
        pkgconfig: "gl",
        fallback:  "GL",
        headers:   &["GL/gl.h"],
    },
    LibSig {
        patterns:  &["glewInit", "glewGetErrorString", "glewExperimental"],
        pkgconfig: "glew",
        fallback:  "GLEW",
        headers:   &["GL/glew.h"],
    },
    LibSig {
        patterns:  &["pthread_create", "pthread_join", "pthread_mutex_lock",
                     "pthread_mutex_unlock", "pthread_mutex_init", "pthread_mutex_destroy",
                     "pthread_cond_wait", "pthread_cond_signal", "pthread_exit"],
        pkgconfig: "pthread",
        fallback:  "pthread",
        headers:   &["pthread.h"],
    },
    LibSig {
        patterns:  &["curl_easy_init", "curl_global_init", "curl_easy_setopt",
                     "curl_easy_perform", "curl_easy_cleanup", "curl_easy_getinfo"],
        pkgconfig: "libcurl",
        fallback:  "curl",
        headers:   &["curl/curl.h"],
    },
    LibSig {
        patterns:  &["gtk_init", "gtk_window_new", "gtk_widget_show",
                     "gtk_main", "gtk_main_quit", "gtk_button_new",
                     "g_signal_connect", "gtk_container_add"],
        pkgconfig: "gtk+-3.0",
        fallback:  "gtk-3",
        headers:   &["gtk/gtk.h"],
    },
    LibSig {
        patterns:  &["initscr", "endwin", "printw", "wrefresh", "getch",
                     "cbreak", "noecho", "keypad", "mvprintw", "attron"],
        pkgconfig: "ncurses",
        fallback:  "ncurses",
        headers:   &["ncurses.h"],
    },
    LibSig {
        patterns:  &["alGenBuffers", "alGenSources", "alSourcePlay",
                     "alDeleteSources", "alcOpenDevice", "alcCreateContext",
                     "alBufferData", "alSourcei", "alListener3f"],
        pkgconfig: "openal",
        fallback:  "openal",
        headers:   &["AL/al.h", "AL/alc.h"],
    },
    LibSig {
        patterns:  &["dlopen", "dlsym", "dlclose", "dlerror"],
        pkgconfig: "dl",
        fallback:  "dl",
        headers:   &["dlfcn.h"],
    },
    LibSig {
        patterns:  &["png_create_read_struct", "png_create_info_struct",
                     "png_read_image", "png_write_image", "png_sig_cmp"],
        pkgconfig: "libpng",
        fallback:  "png",
        headers:   &["png.h"],
    },
    LibSig {
        patterns:  &["deflate", "inflate", "compress", "uncompress",
                     "gzopen", "gzread", "gzclose", "gzwrite"],
        pkgconfig: "zlib",
        fallback:  "z",
        headers:   &["zlib.h"],
    },
    LibSig {
        patterns:  &["sqlite3_open", "sqlite3_exec", "sqlite3_close",
                     "sqlite3_prepare", "sqlite3_step", "sqlite3_finalize"],
        pkgconfig: "sqlite3",
        fallback:  "sqlite3",
        headers:   &["sqlite3.h"],
    },
    LibSig {
        patterns:  &["SSL_new", "SSL_connect", "SSL_read", "SSL_CTX_new",
                     "SSL_library_init", "SSL_write", "SSL_CTX_free"],
        pkgconfig: "openssl",
        fallback:  "ssl",
        headers:   &["openssl/ssl.h"],
    },
    LibSig {
        patterns:  &["SHA256", "MD5", "EVP_DigestInit", "BIO_new",
                     "EVP_EncryptInit", "HMAC"],
        pkgconfig: "libcrypto",
        fallback:  "crypto",
        headers:   &["openssl/evp.h", "openssl/sha.h"],
    },
    LibSig {
        patterns:  &["vkCreateInstance", "vkEnumeratePhysicalDevices",
                     "vkCreateDevice", "vkDestroyInstance", "vkCreateSwapchainKHR"],
        pkgconfig: "vulkan",
        fallback:  "vulkan",
        headers:   &["vulkan/vulkan.h"],
    },
    LibSig {
        patterns:  &["snd_pcm_open", "snd_pcm_writei", "snd_pcm_close",
                     "snd_pcm_prepare", "snd_mixer_open"],
        pkgconfig: "alsa",
        fallback:  "asound",
        headers:   &["alsa/asoundlib.h"],
    },
    LibSig {
        patterns:  &["XOpenDisplay", "XCreateWindow", "XMapWindow",
                     "XDestroyWindow", "XCloseDisplay", "XNextEvent"],
        pkgconfig: "x11",
        fallback:  "X11",
        headers:   &["X11/Xlib.h"],
    },
    LibSig {
        patterns:  &["wl_display_connect", "wl_display_disconnect",
                     "wl_compositor_interface", "wl_registry_bind"],
        pkgconfig: "wayland-client",
        fallback:  "wayland-client",
        headers:   &["wayland-client.h"],
    },
    LibSig {
        patterns:  &["cairo_create", "cairo_surface_create_similar",
                     "cairo_move_to", "cairo_line_to", "cairo_stroke"],
        pkgconfig: "cairo",
        fallback:  "cairo",
        headers:   &["cairo.h"],
    },
    LibSig {
        patterns:  &["FT_Init_FreeType", "FT_New_Face", "FT_Load_Char",
                     "FT_Set_Pixel_Sizes"],
        pkgconfig: "freetype2",
        fallback:  "freetype",
        headers:   &["ft2build.h", "freetype/freetype.h"],
    },
];

// ── Detected link result ──────────────────────────────────────────────────────

pub struct DetectedLib {
    pub name: String,
    pub cflags: Vec<String>,   // -I and -D flags from pkg-config
    pub ldflags: Vec<String>,  // -l and -L flags from pkg-config (or just -lname)
    pub via_pkgconfig: bool,
    pub headers: Vec<String>,  // C headers to #include in generated code
}

/// Scan generated C code and return detected libraries with their GCC flags.
/// Deduplicates against `already_declared` (lib names already in .yourdad / CLI).
pub fn detect_libs_from_c(c_code: &str, already_declared: &[String]) -> Vec<DetectedLib> {
    let mut results = Vec::new();

    for sig in KNOWN_LIBS {
        let hit = sig.patterns.iter().any(|pat| c_code.contains(pat));
        if !hit {
            continue;
        }
        // Skip if already declared
        if already_declared.iter().any(|d| d == sig.fallback || d == sig.pkgconfig) {
            continue;
        }

        // Try pkg-config first
        let headers: Vec<String> = sig.headers.iter().map(|h| h.to_string()).collect();

        if let Some((cflags, ldflags)) = try_pkg_config(sig.pkgconfig) {
            results.push(DetectedLib {
                name: sig.fallback.to_string(),
                cflags,
                ldflags,
                via_pkgconfig: true,
                headers,
            });
        } else {
            // Fallback: just -lname
            results.push(DetectedLib {
                name: sig.fallback.to_string(),
                cflags: vec![],
                ldflags: vec![format!("-l{}", sig.fallback)],
                via_pkgconfig: false,
                headers,
            });
        }
    }

    results
}

/// Run pkg-config and return (cflags, ldflags), or None if not found/available.
fn try_pkg_config(lib: &str) -> Option<(Vec<String>, Vec<String>)> {
    let output = std::process::Command::new("pkg-config")
        .args(&["--cflags", "--libs", lib])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let flags_str = String::from_utf8(output.stdout).ok()?;
    let all: Vec<String> = flags_str.split_whitespace().map(|s| s.to_string()).collect();

    let cflags: Vec<String> = all.iter()
        .filter(|f| f.starts_with("-I") || f.starts_with("-D"))
        .cloned().collect();

    let ldflags: Vec<String> = all.iter()
        .filter(|f| f.starts_with("-l") || f.starts_with("-L"))
        .cloned().collect();

    // Only return if we got something useful
    if cflags.is_empty() && ldflags.is_empty() {
        return None;
    }

    Some((cflags, ldflags))
}

/// Print a summary of auto-detected libraries to stdout.
pub fn print_detected_libs(detected: &[DetectedLib]) {
    if detected.is_empty() {
        return;
    }
    println!("{}", "  🔍 Auto-detected libraries:".cyan());
    for lib in detected {
        if lib.via_pkgconfig {
            println!("     {} {} (via pkg-config: {})",
                "📦".cyan(),
                lib.name,
                lib.ldflags.join(" ").dimmed()
            );
        } else {
            println!("     {} {} (fallback -l flag)",
                "📦".yellow(),
                lib.name,
            );
        }
    }
}

// ── .yourdad config ───────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct YourDadConfig {
    pub sources: Vec<String>,
    pub output: String,
    pub optimization: u8,
    pub c_libs: Vec<String>,
    pub momjoke_files: Vec<String>,
    pub debug: bool,
    pub warnings: bool,
}

impl YourDadConfig {
    /// Parse a .yourdad / .dad file
    pub fn parse(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|_| YourMomError::FileNotFound(path.to_string()))?;
        Self::parse_content(&content, path)
    }

    fn parse_content(content: &str, path: &str) -> Result<Self> {
        let mut cfg = YourDadConfig {
            output: "yourmom_program".to_string(),
            optimization: 2,
            warnings: true,
            ..Default::default()
        };

        #[derive(PartialEq)]
        enum Section { None, Build, Libs, Hats, Left }
        let mut section = Section::None;

        for (lineno, raw) in content.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') { continue; }

            if line.starts_with("daddy_says_build_this")      { section = Section::Build; continue; }
            if line.starts_with("daddy_says_use_protection")  { section = Section::Libs;  continue; }
            if line.starts_with("daddy_says_wear_these_hats") { section = Section::Hats;  continue; }
            if line.starts_with("daddy_left")                 { section = Section::Left;  continue; }
            if line == "}" { section = Section::None; continue; }

            match section {
                Section::Build => {
                    if let Some(val) = extract_value(line, "sources:") {
                        cfg.sources = parse_string_array(val);
                    } else if let Some(val) = extract_value(line, "output:") {
                        cfg.output = val.trim_matches('"').to_string();
                    } else if let Some(val) = extract_value(line, "optimization:") {
                        cfg.optimization = val.trim().parse().unwrap_or(2);
                    }
                }
                Section::Libs => {
                    if let Some(val) = extract_value(line, "c_libs:") {
                        cfg.c_libs = parse_string_array(val);
                    }
                }
                Section::Hats => {
                    let s = line.trim_matches('"');
                    if !s.is_empty() && s.ends_with(".momjoke") {
                        cfg.momjoke_files.push(s.to_string());
                    }
                }
                Section::Left => {
                    if let Some(val) = extract_value(line, "debug:") {
                        cfg.debug = val.trim() == "true";
                    } else {
                        let _ = lineno;
                    }
                }
                Section::None => {}
            }
        }

        if cfg.sources.is_empty() {
            return Err(YourMomError::SyntaxError(format!(
                "daddy left without telling us what to build in '{}' — no sources: found",
                path
            )));
        }

        Ok(cfg)
    }

    /// Base GCC flags from config (optimization, debug, warnings, declared c_libs).
    /// Does NOT include auto-detected libs — those are added separately.
    pub fn gcc_flags(&self) -> Vec<String> {
        let mut flags = vec![
            format!("-O{}", self.optimization),
            "-I.".to_string(),
        ];
        if self.debug   { flags.push("-g".to_string()); }
        if self.warnings { flags.push("-Wall".to_string()); }
        for lib in &self.c_libs {
            flags.push(format!("-l{}", lib));
        }
        flags
    }

    /// Produce a human-readable dependency tree string
    pub fn format_tree(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("output: {}\n", self.output));
        out.push_str("sources:\n");
        for s in &self.sources { out.push_str(&format!("  ├─ {}\n", s)); }
        if !self.c_libs.is_empty() {
            out.push_str("c_libs:\n");
            for lib in &self.c_libs { out.push_str(&format!("  ├─ -l{}\n", lib)); }
        }
        if !self.momjoke_files.is_empty() {
            out.push_str("momjoke:\n");
            for jk in &self.momjoke_files { out.push_str(&format!("  ├─ {}\n", jk)); }
        }
        out
    }
}

fn extract_value<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    line.strip_prefix(key).map(|s| s.trim())
}

fn parse_string_array(s: &str) -> Vec<String> {
    let inner = s.trim_start_matches('[').trim_end_matches(']').trim();
    inner.split(',')
        .map(|p| p.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

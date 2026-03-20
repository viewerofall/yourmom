# .yourmom — VQ (Version 3: Quantum)

**"Holy C but what the fuck happened"**

A quantum-enabled esoteric programming language with your mom jokes as error messages, profanity-laced diagnostics, and more number types than mathematicians know what to do with. Compiles to C via GCC.

---

## What Makes This Unique

1. **Quantum Superposition Variables** — Variables exist in multiple states until observed, then collapse and stay collapsed
2. **Every Number Type Ever** — integers, reals, rationals, irrationals, imaginary/complex, quaternions, and transfinite numbers
3. **Heisenberg Variables** — Re-collapse to a new value *every single time* you observe them
4. **Entangled Variables** — Two variables that always share the same collapsed value
5. **Schrödinger's If** — A conditional that ignores its own condition and flips a quantum coin instead
6. **Family-Themed CLI** — `childmake`, `abortion`, `divorce`, `deadbeat`, `custody`
7. **Your Mom Joke Errors** — Every compile error, runtime crash, and segfault is a yo mama joke
8. **`.yourdad` Build System** — Like Cargo.toml but cursed
9. **`.momjoke` Standard Library** — Shorthand aliases for the whole stdlib

---

## Quick Start

```bash
cd yourmom-rust
cargo build --release

# Compile a .yourmom file
./target/release/yourmom childmake hello.yourmom

# Compile and run immediately
./target/release/yourmom run hello.yourmom

# Clean build artifacts
./target/release/yourmom abortion

# Help
./target/release/yourmom --help
```

---

## Language Reference

### Variables

```yourmom
yo x = 42             // regular int
yo name = "hello"     // string
```

### Functions

```yourmom
yo my_func(a, b) {
    did_your_mom a + b
}

yo mama_main() {
    yo result = doing_your_mom my_func(10, 20)
    ymf(result)
}
```

`mama_main` is the entry point. `did_your_mom` is return. `doing_your_mom` is a function call expression. `ymf` is print.

### Shortcuts (`.momjoke` stdlib)

| Shorthand | Expands to |
|-----------|-----------|
| `ymf` | `yo_mama_so_fat` (print) |
| `ymd` | `yo_mama_so_dumb` (panic) |
| `ymsu` | `yo_mama_so_ugly` (unsafe) |
| `dym` | `doing_your_mom` (call) |
| `ret` | `did_your_mom` (return) |

### Control Flow

```yourmom
// if / else
mama_said x > 0 {
    ymf("positive")
} mama_lied {
    ymf("not positive")
}

// while
mama_keeps_saying x > 0 {
    x = x - 1
}

// Schrödinger's if — ignores condition, flips quantum coin
mama_said_maybe {
    ymf("maybe this")
} mama_lied {
    ymf("maybe that")
}

// goto
mama_forgot my_label
// ...
my_label:
    ymf("yo mama teleported here")
```

### try / catch

Uses `setjmp`/`longjmp` under the hood:

```yourmom
yo_daddy_issues {
    ymf("risky shit in here")
} mama_caught {
    ymf("caught the drama")
}
```

---

## Quantum Number Types

Every quantum variable is in superposition until first observed, then collapses and stays collapsed (except Heisenberg).

### Discrete Superposition

```yourmom
yo x = 1 | 2 | 3 | 4    // collapses to one of these
ymf(x)                   // prints collapsed value
ymf(x)                   // same value — already collapsed
```

### All Integers

```yourmom
yo x = int_all    // any 32-bit integer
ymf(x)
```

### All Real Numbers

```yourmom
yo x = real_all   // any double-precision float
ymf(x)
```

### All Rational Numbers

```yourmom
yo x = rational_all   // collapses to p/q (fully reduced fraction)
ymf(x)                // prints e.g. "355/113"
```

### All Irrational Numbers

```yourmom
yo x = irrational_all   // collapses to a famous irrational constant
ymf(x)                  // prints e.g. "pi ~= 3.14159265358979"
```

Available constants: π, e, √2, √3, √5, φ (golden ratio), ln(2), √7, π/2, γ (Euler-Mascheroni), ζ(3) (Apéry), log₁₀2, δ (Feigenbaum), ∛36, ρ (plastic number)

### All Imaginary / Complex Numbers

```yourmom
yo x = imaginary_all   // collapses to a+bi (b ≠ 0)
ymf(x)                 // prints e.g. "3.5+2.1i"
```

### Quaternions

```yourmom
yo q = quaternion_all   // collapses to w+xi+yj+zk
ymf(q)                  // prints e.g. "1.2+3.4i+5.6j-7.8k"
```

### Transfinite Numbers

```yourmom
yo t = transfinite_all   // collapses to a transfinite cardinal/ordinal
ymf(t)                   // prints e.g. "omega^omega"
```

Available: ℵ₀, ℵ₁, ℵ₂, ω, ω+1, ω², ω^ω, ε₀

### Heisenberg Variables

Re-collapses to a new random value **every time** it is observed:

```yourmom
yo x = heisenberg(10 | 20 | 30)
ymf(x)    // might print 10
ymf(x)    // might print 30
ymf(x)    // might print 20
```

### Entangled Variables

Always collapses to the same value as another variable:

```yourmom
yo x = 1 | 2 | 3
yo y = entangled_with x
ymf(x)    // e.g. 2
ymf(y)    // also 2, guaranteed
```

### Quantum Sort

Sorts a superposition and collapses to the smallest value:

```yourmom
yo sorted = quantum_sort(5 | 3 | 1 | 4 | 2)
// prints: [1, 2, 3, 4, 5] -> 1
```

---

## Extra Statements

### Loud Print

```yourmom
yo_mama_so_loud("hello world")
// prints: 🔊 BITCH: HELLO WORLD
```

### Paranoid Assertion

50% chance of actually checking the assertion at runtime:

```yourmom
yo_mama_so_paranoid(x > 0)
// might explode with: PARANOID SHIT FAILED
```

### Lazy Evaluation

Value is only computed on first access:

```yourmom
yo x = yo_mama_so_lazy(100 + 200)
ymf(x)    // computed here, prints 300
ymf(x)    // cached, prints 300
```

### Unsafe / Inline Assembly

```yourmom
yo_mama_so_ugly {
    __asm__("nop")
}
```

---

## Macros

Define a macro at the top level:

```yourmom
yo_mama_so_meta my_print(x) = ymf(x)

yo mama_main() {
    my_print("hello from macro")
}
```

### Self-Modifying Source

At runtime, rewrites its own source file with a yo mama comment prepended:

```yourmom
yo_mama_so_meta rewrite_self("myprogram.yourmom")
```

---

## CLI Commands

```bash
yourmom childmake file.yourmom          # Compile
yourmom childmake file.yourmom -o name  # Compile with output name
yourmom run file.yourmom                # Compile and run
yourmom abortion                        # Clean build artifacts
yourmom abortion --deep                 # Deep clean
yourmom divorce binary_name             # Remove binary
yourmom family-tree project.dad         # Show dependency tree
yourmom deadbeat project.dad            # Find unused deps
yourmom custody list                    # Show .momjoke files
yourmom custody add file.momjoke        # Add .momjoke
yourmom custody remove file.momjoke     # Remove .momjoke
```

---

## Error Messages

Every error is a yo mama joke. Examples:

- **Segfault:** `💀 WHAT IN THE ACTUAL FUCK (signal 11): Yo mama so fat, her ass caused a goddamn stack overflow`
- **Parse error:** `expected Yo, got Identifier — goddamn it`
- **File not found:** `Yo mama so lost, she can't find file: foo.yourmom`
- **GCC failed:** `Yo mama so broke, GCC refused to compile`

---

## Project Structure

```
yourmom-rust/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Public API (compile_to_c, compile_to_binary)
│   ├── lexer/
│   │   ├── mod.rs           # Tokenizer
│   │   ├── token.rs         # Token enum
│   │   └── shortcuts.rs     # .momjoke shorthand expansion
│   ├── parser/
│   │   ├── mod.rs           # Parser
│   │   └── ast.rs           # AST types
│   ├── codegen/
│   │   └── mod.rs           # C code generator
│   ├── errors/
│   │   └── mod.rs           # Error types + yo mama joke database
│   ├── build_system/
│   │   └── mod.rs           # .yourdad config parser
│   ├── cli/
│   │   └── mod.rs           # Command implementations
│   └── runtime/
│       ├── mod.rs           # Embeds quantum_runtime.h
│       └── quantum_runtime.h  # Full quantum C runtime
└── *.yourmom                # Example/test files
```

---

## How It Works

1. `.yourmom` source is lexed into tokens (shortcuts expanded inline)
2. Tokens are parsed into an AST
3. AST is walked by the codegen, emitting C code
4. `quantum_runtime.h` is written to disk
5. GCC compiles `output.c` against the runtime
6. Binary runs

The quantum runtime is a pure C header with no external dependencies. Superposition is implemented as an array of possible values with lazy collapse via `rand()`. Heisenberg variables skip the collapse cache. Try-catch uses `setjmp`/`longjmp`. Quaternions, transfinites, rationals, irrationals, and complex numbers each have their own struct type with a print function.

---

## Full Example

```yourmom
// VQ v3 showcase
yo mama_main() {

    // Hello
    ymf("Hello, World!")

    // Quantum superposition
    yo coin = 0 | 1
    mama_said coin == 1 {
        ymf("heads")
    } mama_lied {
        ymf("tails")
    }

    // All number types
    yo a = int_all
    yo b = real_all
    yo c = rational_all
    yo d = irrational_all
    yo e = imaginary_all
    yo q = quaternion_all
    yo t = transfinite_all
    ymf(a)
    ymf(b)
    ymf(c)
    ymf(d)
    ymf(e)
    ymf(q)
    ymf(t)

    // Heisenberg — different every time
    yo h = heisenberg(10 | 20 | 30)
    ymf(h)
    ymf(h)
    ymf(h)

    // Entanglement
    yo x = 1 | 2 | 3
    yo y = entangled_with x
    ymf(x)
    ymf(y)

    // Quantum sort
    yo sorted = quantum_sort(5 | 3 | 1 | 4 | 2)

    // Schrödinger's if
    mama_said_maybe {
        yo_mama_so_loud("branch A")
    } mama_lied {
        yo_mama_so_loud("branch B")
    }

    // Lazy eval
    yo lazy_val = yo_mama_so_lazy(999)
    ymf(lazy_val)

    // Try-catch
    yo_daddy_issues {
        ymf("living dangerously")
    } mama_caught {
        ymf("caught the drama")
    }

    // Paranoid assertion
    yo_mama_so_paranoid(a > 0)

    // goto
    mama_forgot skip_this
    ymf("this line is skipped")
    skip_this:
        ymf("teleported here")

}
```

---

## License

WTFPL — Do what the fuck you want. I'm not your mom.

## Credits

Original concept and Python v2 transpiler by **viewerofall**.
VQ v3.0.0 (Rust implementation) built on CachyOS with Niri compositor.

*Quantum mechanics meets your mom jokes. What in the actual fuck were we thinking.*

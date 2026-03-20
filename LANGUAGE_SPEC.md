# .yourmom Programming Language - Complete Specification

**Version:** VQ 3.0 (Version Quantum)  
**Tagline:** "Holy C but what the fuck happened"  
**Creator:** viewerofall (Bill / abyss)  
**Repository:** https://github.com/viewerofall/yourmom-lang

---

## Table of Contents

1. [Overview](#overview)
2. [Language Philosophy](#language-philosophy)
3. [Compilation Architecture](#compilation-architecture)
4. [Syntax Reference](#syntax-reference)
5. [Quantum Mechanics System](#quantum-mechanics-system)
6. [Advanced Features](#advanced-features)
7. [Standard Library (jksmpl.momjoke)](#standard-library)
8. [CLI Commands](#cli-commands)
9. [Build System (.yourdad)](#build-system)
10. [Error Handling](#error-handling)
11. [C Interoperability](#c-interoperability)
12. [Example Programs](#example-programs)
13. [Implementation Details](#implementation-details)
14. [Project Ecosystem](#project-ecosystem)

---

## Overview

.yourmom is a quantum-enabled esoteric programming language that transpiles to C. It combines:
- **Quantum mechanics** (superposition, entanglement, Heisenberg uncertainty)
- **Your mom jokes** (error messages, CLI commands)
- **Advanced number theory** (8+ number type categories)
- **Modern language features** (try/catch, macros, lazy evaluation)
- **Self-modifying code capabilities**
- **C-level performance** (compiles to native binaries via GCC)

**What makes it unique:**
- First language with Heisenberg variables (change on every read)
- Only language with transfinite number types (ℵ₀, ω, ε₀)
- Quantum superposition with deterministic collapse
- Family-themed CLI (childmake, abortion, divorce)
- Self-modifying source code support
- Your mom jokes as error messages

---

## Language Philosophy

### Design Principles:

1. **Quantum First** - Variables can exist in multiple states simultaneously
2. **No Implicit Anything** - Explicit quantum collapse, explicit memory management
3. **Fast as C** - Transpiles to C, no runtime overhead except quantum mechanics
4. **Intentionally Cursed** - Your mom jokes everywhere, family-themed terminology
5. **Actually Functional** - Despite the humor, it's a real, working language

### Target Use Cases:

- **Esoteric programming challenges**
- **Quantum algorithm simulations**
- **Educational demonstrations of quantum concepts**
- **Meme-tier system utilities**
- **GUI applications** (via raylib/GTK bindings)
- **Replacement for systemd** (daddy_init project)

---

## Compilation Architecture

```
.yourmom source file
    ↓
Lexer (shortcuts.rs expands jksmpl.momjoke)
    ↓
Tokens with position tracking
    ↓
Parser (ast.rs builds syntax tree)
    ↓
AST with quantum variable tracking
    ↓
Codegen (emits C code)
    ↓
output.c + quantum_runtime.h
    ↓
GCC compilation
    ↓
Native binary
```

### Components:

**Transpiler (Rust):**
- `src/lexer/` - Tokenization (230 lines)
- `src/parser/` - AST generation (596 lines)
- `src/codegen/` - C code emission (605 lines)
- `src/errors/` - Your mom joke error system
- `src/runtime/` - Embeds quantum_runtime.h

**Runtime (C):**
- `quantum_runtime.h` - Quantum mechanics implementation
- `libmama.c` - System call wrappers (optional)

**Total:** ~1400 lines of production code

---

## Syntax Reference

### Variables

```yourmom
yo x = 42              // Regular integer
yo name = "hello"      // String literal
yo pi = 3.14           // Float (real number)
```

### Functions

```yourmom
yo my_function(a, b) {
    yo result = a + b
    did_your_mom result
}

yo mama_main() {
    yo sum = doing_your_mom my_function(10, 20)
    yo_mama_so_fat(sum)
}
```

**Entry point:** `mama_main()` (equivalent to C's `main`)  
**Return:** `did_your_mom value`  
**Call:** `doing_your_mom function_name(args)`  
**Print:** `yo_mama_so_fat(value)` or shorthand `ymf(value)`

### Control Flow

**If/Else:**
```yourmom
mama_said x > 0 {
    ymf("positive")
} mama_lied {
    ymf("not positive")
}
```

**While Loop:**
```yourmom
mama_keeps_saying counter < 10 {
    ymf(counter)
    counter = counter + 1
}
```

**For Loop (via while):**
```yourmom
yo i = 0
mama_keeps_saying i < 10 {
    ymf(i)
    i = i + 1
}
```

### Operators

**Arithmetic:** `+`, `-`, `*`, `/`, `%`  
**Comparison:** `==`, `!=`, `<`, `>`, `<=`, `>=`  
**Quantum:** `|` (superposition separator)

### Comments

```yourmom
// Single-line comment
// Multi-line not supported (use multiple //)
```

---

## Quantum Mechanics System

### 1. Basic Superposition

Variables can exist in multiple states until observed:

```yourmom
yo x = 10 | 20 | 30 | 40 | 50
ymf(x)    // Collapses to one value (e.g., 30)
ymf(x)    // Prints same value (30)
ymf(x)    // Still 30 - already collapsed
```

**Mechanics:**
- First read triggers collapse (random selection)
- Collapsed value is cached
- All subsequent reads return same value
- Uses seeded RNG for determinism

### 2. Number Type Categories

#### int_all - All Integers
```yourmom
yo x = int_all
ymf(x)    // Prints random 32-bit integer
```

Collapses to any value in range `[-2147483648, 2147483647]`

#### real_all - All Real Numbers
```yourmom
yo x = real_all
ymf(x)    // Prints random double-precision float
```

Collapses to any double-precision floating point value

#### rational_all - All Rational Numbers
```yourmom
yo x = rational_all
ymf(x)    // Prints fraction e.g., "355/113"
```

Collapses to a fully-reduced fraction `p/q` where `gcd(p,q) = 1`

#### irrational_all - Famous Irrational Constants
```yourmom
yo x = irrational_all
ymf(x)    // Prints e.g., "pi ~= 3.14159265358979"
```

Collapses to one of:
- π (pi)
- e (Euler's number)
- √2, √3, √5, √7
- φ (golden ratio)
- ln(2)
- γ (Euler-Mascheroni constant)
- ζ(3) (Apéry's constant)
- log₁₀2
- δ (Feigenbaum constant)
- ∛36
- ρ (plastic number)

#### imaginary_all - Complex Numbers
```yourmom
yo x = imaginary_all
ymf(x)    // Prints e.g., "3.5+2.1i"
```

Collapses to `a+bi` where `b ≠ 0` (pure imaginary or complex)

#### quaternion_all - Quaternions
```yourmom
yo q = quaternion_all
ymf(q)    // Prints e.g., "1.2+3.4i+5.6j-7.8k"
```

Collapses to `w+xi+yj+zk` (4D rotation representation)

#### transfinite_all - Infinite Cardinals/Ordinals
```yourmom
yo t = transfinite_all
ymf(t)    // Prints e.g., "omega^omega"
```

Collapses to one of:
- ℵ₀ (aleph-null, countable infinity)
- ℵ₁ (aleph-one)
- ℵ₂ (aleph-two)
- ω (omega, first infinite ordinal)
- ω+1
- ω²
- ω^ω
- ε₀ (epsilon-nought)

### 3. Heisenberg Variables

Re-collapse to a new value **every time** they're observed:

```yourmom
yo x = heisenberg(10 | 20 | 30 | 40)
ymf(x)    // Might print 20
ymf(x)    // Might print 40
ymf(x)    // Might print 10
```

**Use case:** True randomness, non-deterministic algorithms

**Limitation:** Cannot be used in assertions (`yo_mama_so_paranoid`)

### 4. Entangled Variables

Two variables that always share the same collapsed value:

```yourmom
yo x = 1 | 2 | 3 | 4 | 5
yo y = entangled_with x

ymf(x)    // e.g., 3
ymf(y)    // Also 3, guaranteed
```

**Mechanics:**
- Entangled variable references the same collapse state
- Both always return identical values
- Simulates quantum entanglement

### 5. Quantum Sort

Sort a superposition and collapse to the minimum:

```yourmom
yo sorted = quantum_sort(5 | 3 | 1 | 4 | 2)
ymf(sorted)    // Prints: [1, 2, 3, 4, 5] -> 1
```

Returns the smallest value after sorting

---

## Advanced Features

### 1. Schrödinger's If (mama_said_maybe)

Conditional that **ignores** its condition and flips a quantum coin:

```yourmom
mama_said_maybe {
    ymf("Branch A")
} mama_lied {
    ymf("Branch B")
}
```

**Behavior:** 50/50 chance of executing either branch, regardless of any condition

**Use case:** Non-deterministic branching, chaos engineering

### 2. Goto (mama_forgot)

Jump to a label:

```yourmom
mama_forgot skip_this
ymf("This line is skipped")
ymf("This too")
skip_this:
    ymf("Execution starts here")
```

**Warning:** Goto bypasses all intermediate code

### 3. Try/Catch (yo_daddy_issues / mama_caught)

Exception handling using `setjmp`/`longjmp`:

```yourmom
yo_daddy_issues {
    ymf("Risky code here")
    // Might throw
} mama_caught {
    ymf("Exception caught!")
}
```

**Mechanics:** Uses C's `setjmp`/`longjmp` under the hood

### 4. Lazy Evaluation (yo_mama_so_lazy)

Value computed only on first access:

```yourmom
yo x = yo_mama_so_lazy(expensive_computation())
ymf("Not computed yet")
ymf(x)    // Computed here
ymf(x)    // Cached, not recomputed
```

### 5. Loud Print (yo_mama_so_loud)

Uppercase output with emoji:

```yourmom
yo_mama_so_loud("hello world")
// Output: 🔊 BITCH: HELLO WORLD
```

### 6. Paranoid Assertion (yo_mama_so_paranoid)

Assertion with 50% chance of actually checking:

```yourmom
yo x = 42
yo_mama_so_paranoid(x > 0)
// 50% chance: passes
// 50% chance: checks assertion, might fail
```

**Use case:** Probabilistic testing, chaos mode

### 7. Macros (yo_mama_so_meta)

Define compile-time macros:

```yourmom
yo_mama_so_meta my_print(x) = ymf(x)

yo mama_main() {
    my_print("Hello from macro!")
}
```

### 8. Self-Modifying Code (rewrite_self)

Rewrites source file at runtime:

```yourmom
yo_mama_so_meta rewrite_self("my_program.yourmom")
// Prepends "// yo mama so meta, she rewrote herself" to source
```

**Warning:** Actually modifies the source file on disk

### 9. Unsafe Blocks (yo_mama_so_ugly)

Raw inline assembly:

```yourmom
yo_mama_so_ugly {
    __asm__("nop")
    __asm__("mov $42, %rax")
}
```

**Requirement:** Must contain at least one `__asm__` statement

---

## Standard Library (jksmpl.momjoke)

### Core Shortcuts

| Shorthand | Full Form | Description |
|-----------|-----------|-------------|
| `ymf` | `yo_mama_so_fat` | Print with newline |
| `ymd` | `yo_mama_so_dumb` | Panic/abort |
| `ymsu` | `yo_mama_so_ugly` | Unsafe block |
| `dym` | `doing_your_mom` | Function call |
| `ret` | `did_your_mom` | Return statement |

### I/O Functions

| Function | Description |
|----------|-------------|
| `yo_mama_so_fat(x)` | Print value with newline |
| `yo_mama_so_loud(s)` | Print uppercase with emoji |
| `yo_mama_so_nosy()` | Read integer from stdin |
| `yo_mama_so_quiet()` | Read string from stdin |

### Math Functions

| Function | Description |
|----------|-------------|
| `yo_mama_so_random()` | Random integer (rand()) |
| `yo_mama_so_round(x)` | Round to nearest integer |
| `yo_mama_so_square(x)` | Square (x²) |
| `yo_mama_so_thicc(x)` | Absolute value |
| `yo_mama_so_wide(x)` | Square root |

### String Functions

| Function | Description |
|----------|-------------|
| `yo_mama_so_long(s)` | String length |
| `yo_mama_so_cheap_copy(dst, src)` | String copy |
| `yo_mama_so_connected(dst, src)` | String concatenation |

### Memory Functions

| Function | Description |
|----------|-------------|
| `yo_mama_so_fat_malloc(size)` | Allocate memory (quantum-aware) |
| `yo_mama_so_cheap_free(ptr)` | Free memory |

### Quantum Functions

| Function | Description |
|----------|-------------|
| `heisenberg(vals)` | Create Heisenberg variable |
| `entangled_with var` | Create entangled variable |
| `quantum_sort(vals)` | Sort and return minimum |
| `yo_mama_observe(var)` | Force collapse |
| `yo_mama_collapsed(var)` | Check if collapsed |

---

## CLI Commands

### Family-Themed Interface

```bash
yourmom childmake file.yourmom         # Compile
yourmom childmake file.yourmom -o name # Compile with output name
yourmom run file.yourmom                # Compile and run
yourmom abortion                        # Clean build artifacts
yourmom abortion --deep                 # Deep clean (quantum caches)
yourmom divorce binary_name             # Remove executable
yourmom family-tree project.dad         # Show dependency tree
yourmom deadbeat project.dad            # Find unused dependencies
yourmom custody list                    # Show loaded .momjoke files
yourmom custody add file.momjoke        # Add .momjoke dependency
yourmom custody remove file.momjoke     # Remove .momjoke dependency
```

### Command Etymology

| Command | Meaning | Joke |
|---------|---------|------|
| `childmake` | Compile | "Daddy knocked up the source code" |
| `abortion` | Clean | "Abort the build artifacts" |
| `divorce` | Remove binary | "Divorce from the executable" |
| `family-tree` | Dep graph | "Show family relationships" |
| `deadbeat` | Unused deps | "Find deadbeat dependencies" |
| `custody` | Manage libs | "Custody battle over .momjoke files" |

---

## Build System (.yourdad)

Project configuration files (like Cargo.toml):

```yourdad
# project.yourdad

daddy_says_build_this {
    sources: ["main.yourmom", "lib.yourmom"]
    output: "my_app"
    optimization: 2
}

daddy_says_use_protection {
    c_libs: ["m", "pthread", "raylib"]
}

daddy_says_wear_these_hats {
    "jksmpl.momjoke"
    "custom_utils.momjoke"
}

daddy_left {
    # Exclude from build
    unsafe_optimizations: true
}
```

### Sections

- `daddy_says_build_this` - Build configuration
- `daddy_says_use_protection` - C library dependencies
- `daddy_says_wear_these_hats` - .momjoke imports
- `daddy_left` - Exclusions/optional configs

---

## Error Handling

### Contextual Your Mom Jokes

Every error is a contextually-appropriate your mom joke:

**Parse Error:**
```
💀 ERROR: Expected '}', got end of file
  ┌─ hello.yourmom:5:18
  │
5 │ yo mama_main() {
  │                ^ Yo mama so dumb, she forgot to close this brace
  │
  = help: Add '}' at the end of the function
```

**Type Error:**
```
💀 ERROR: Cannot mix quantum and classical types
  ┌─ test.yourmom:7:10
  │
7 │ yo sum = x + 42
  │          ^^^^^
  │          Yo mama so mixed up, she tried quantum + classical math
  │
  = help: Wrap 42 in superposition: (42) or use quantum_observe(x)
```

**Heisenberg Assertion:**
```
💀 ERROR: Heisenberg variable used in assertion
  ┌─ test.yourmom:10:21
  │
10 │ yo_mama_so_paranoid(h > 0)
   │                     ^
   │     Yo mama so uncertain, she asserted a Heisenberg variable
   │
   = help: Heisenberg variables change on every read; can't be asserted
```

**Segfault (Runtime):**
```
💀 WHAT IN THE ACTUAL FUCK (signal 11):
Yo mama so fat, her ass caused a goddamn stack overflow

Call Trace:
  yo_mama_so_fat+0x0/0x0
  doing_your_mom+0x69/0x420
  mama_main+0x1/0x1
```

---

## C Interoperability

### Calling C Functions

```yourmom
// Declare external C function
yo_mama_so_ugly {
    __asm__(".extern printf")
}

yo mama_main() {
    dym printf("Hello from C!\n")
}
```

### Using C Libraries

**In .yourdad:**
```yourdad
daddy_says_use_protection {
    c_libs: ["raylib", "m", "pthread"]
}
```

**In code:**
```yourmom
// Wrapper functions defined in C
dym ym_init_window(800, 600, "My App")
dym ym_draw_text("Hello", 100, 100, 20, 255, 255, 255)
```

---

## Example Programs

### Hello World

```yourmom
yo mama_main() {
    ymf("Hello, World!")
}
```

### Quantum Coin Flip

```yourmom
yo mama_main() {
    yo coin = 0 | 1
    
    mama_said coin == 1 {
        ymf("Heads!")
    } mama_lied {
        ymf("Tails!")
    }
}
```

### Fibonacci (Recursive)

```yourmom
yo fib(n) {
    mama_said n <= 1 {
        did_your_mom n
    }
    did_your_mom dym fib(n-1) + dym fib(n-2)
}

yo mama_main() {
    yo result = dym fib(10)
    ymf(result)  // 55
}
```

### Factorial with All Number Types

```yourmom
yo mama_main() {
    yo a = int_all
    yo b = real_all
    yo c = rational_all
    yo d = irrational_all
    yo e = imaginary_all
    yo q = quaternion_all
    yo t = transfinite_all
    
    ymf(a)  // e.g., 42
    ymf(b)  // e.g., 3.14159
    ymf(c)  // e.g., 22/7
    ymf(d)  // e.g., pi ~= 3.14159265358979
    ymf(e)  // e.g., 3.5+2.1i
    ymf(q)  // e.g., 1.2+3.4i+5.6j-7.8k
    ymf(t)  // e.g., omega^omega
}
```

---

## Implementation Details

### Quantum Runtime Implementation (C)

**Data Structure:**
```c
typedef struct {
    int* values;        // Array of possible values
    int count;          // Number of possibilities
    int collapsed_idx;  // -1 if superposition, else index
    int collapsed_val;  // Cached collapsed value
} quantum_int;
```

**Collapse Mechanism:**
```c
int _quantum_observe(quantum_int* q) {
    if (q->collapsed_idx == -1) {
        _quantum_init_rng();
        q->collapsed_idx = rand() % q->count;
        q->collapsed_val = q->values[q->collapsed_idx];
    }
    return q->collapsed_val;
}
```

**Heisenberg Variables:**
```c
typedef struct {
    int* values;
    int count;
    // NO collapsed_idx - always re-randomizes
} quantum_heisenberg;

int _heisenberg_observe(quantum_heisenberg* h) {
    _quantum_init_rng();
    return h->values[rand() % h->count];
}
```

### Codegen Strategy

**Regular Variable:**
```yourmom
yo x = 42
```
→ Generates:
```c
int x = 42;
```

**Quantum Variable:**
```yourmom
yo x = 10 | 20 | 30
```
→ Generates:
```c
quantum_int x = SUPERPOSITION(10, 20, 30);
```

**Heisenberg Variable:**
```yourmom
yo h = heisenberg(1 | 2 | 3)
```
→ Generates:
```c
quantum_heisenberg h = HEISENBERG(1, 2, 3);
```

---

## Project Ecosystem

### Official Projects

1. **yourmom-rust** - Main compiler (Rust)
2. **Yo Mama Clicker** - GUI game demo (raylib)
3. **Yo Mama Linux** - Arch-based distro (planned)
4. **daddy_init** - Init system replacement (planned)

### Community Extensions

- Custom .momjoke libraries
- C wrapper libraries for various libs
- Build tools and integrations

### Package Repositories

- **AUR** - `yourmom` package
- **GitHub** - Git-based package registry (yomama-packages)
- **Flatpak-style** - Fork, add package, submit PR

---

## Feature Comparison

| Feature | .yourmom | Rust | C | Python | Brainfuck |
|---------|----------|------|---|--------|-----------|
| Quantum superposition | ✅ | ❌ | ❌ | ❌ | ❌ |
| Heisenberg variables | ✅ | ❌ | ❌ | ❌ | ❌ |
| Transfinite numbers | ✅ | ❌ | ❌ | ❌ | ❌ |
| Goto support | ✅ | ❌ | ✅ | ❌ | N/A |
| Try/catch | ✅ | ✅ | ❌ | ✅ | ❌ |
| Self-modifying code | ✅ | ❌ | ❌ | ✅ | ❌ |
| Compiles to native | ✅ | ✅ | ✅ | ❌ | Varies |
| Your mom jokes | ✅ | ❌ | ❌ | ❌ | ❌ |

---

## Performance Characteristics

**Compilation:**
- Transpile: ~10ms (Python) / ~1ms (Rust)
- GCC: ~100ms (typical program)
- Total: ~110ms for Hello World

**Runtime:**
- Regular operations: C speed (native)
- Quantum collapse: ~1μs per variable (first read)
- Subsequent reads: Cached, ~0 overhead
- Heisenberg reads: ~1μs per read (re-randomize)

**Memory:**
- Regular int: 4 bytes
- Quantum int: 24 bytes + value array
- Heisenberg int: 16 bytes + value array

---

## Future Roadmap

**v3.1 (Current):**
- ✅ Full Rust transpiler
- ✅ 8 quantum number types
- ✅ GUI app support (raylib)

**v3.2 (Next):**
- Enhanced error diagnostics (Rust-style)
- REPL mode
- Debugger integration

**v4.0 (Future):**
- LLVM backend (optional)
- Quantum algorithms stdlib
- VSCode extension

**v5.0 (Dream):**
- Yo Mama Linux distro release
- Package manager integration
- Community package registry

---

## License

**WTFPL** - Do What The Fuck You Want To Public License

"I'm not your mom. Do whatever you want."

---

## Credits

**Creator:** viewerofall (Bill / abyss)  
**Platform:** Built on CachyOS with Niri compositor  
**Aesthetic:** Purple TWM / OneShot inspired  
**Influences:** HolyC, Brainfuck, Quantum computing, Cookie Clicker

---

**"Yo mama so quantum, she exists in multiple states simultaneously."**

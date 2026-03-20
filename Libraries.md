# .yourmom Standard Libraries - Complete Reference

**The full ecosystem of quality-of-life features, shortcuts, and quantum runtime functions**

Version: VQ 3.0  
Last Updated: March 2026

---

## Table of Contents

1. [Overview](#overview)
2. [jksmpl.momjoke - Core Standard Library](#jksmplmomjoke)
3. [qol.momjoke - Quality of Life Extensions](#qolmomjoke)
4. [quantum_runtime.h - Quantum Mechanics Engine](#quantum_runtimeh)
5. [Quick Reference Tables](#quick-reference)
6. [Usage Examples](#usage-examples)

---

## Overview

The .yourmom language comes with three main libraries that make development easier:

### **jksmpl.momjoke** - Core Standard Library
- Essential shortcuts (ymf, dym, ret)
- Basic I/O, math, string, memory functions
- Quantum mechanics keywords
- **Auto-loaded by default**

### **qol.momjoke** - Quality of Life Extensions
- Formatted printing (printf-style)
- File I/O operations
- Time/sleep functions
- String utilities (search, uppercase, lowercase)
- Shorter aliases for common operations
- **Load explicitly in .yourdad file**

### **quantum_runtime.h** - Quantum Mechanics Engine
- Superposition implementation
- Quantum variable types (int, float, rational, etc.)
- Observation/collapse mechanics
- Error handling with your mom jokes
- **Auto-included by compiler**

---

## jksmpl.momjoke

**"Just Keep Saying 'Mama' Periodically Library"**

The core standard library that provides essential shortcuts and functions.

### Core Shortcuts

These make writing .yourmom code less painful:

| Shorthand | Expands To | Purpose |
|-----------|------------|---------|
| `ymf` | `yo_mama_so_fat` | Print with newline |
| `ymd` | `yo_mama_so_dumb` | Panic/abort program |
| `ymsu` | `yo_mama_so_ugly` | Unsafe block |
| `dym` | `doing_your_mom` | Function call |
| `ret` | `did_your_mom` | Return statement |

**Examples:**
```yourmom
// Instead of this:
yo_mama_so_fat("Hello")
yo result = doing_your_mom add(5, 10)
did_your_mom result

// Write this:
ymf("Hello")
yo result = dym add(5, 10)
ret result
```

---

### I/O Functions

#### Output Functions

**`yo_mama_so_fat(x)` / `ymf(x)`**
- Prints value with newline
- Works with: integers, strings, quantum variables
```yourmom
ymf("Hello world")
ymf(42)
yo x = 10 | 20 | 30
ymf(x)  // Prints collapsed value
```

**`yo_mama_so_loud(s)`**
- Prints uppercase with 🔊 emoji
- Adds "BITCH: " prefix
```yourmom
yo_mama_so_loud("hello world")
// Output: 🔊 BITCH: HELLO WORLD
```

#### Input Functions

**`yo_mama_so_nosy()`**
- Reads integer from stdin
- Blocks until user enters number
```yourmom
ymf("Enter your age:")
yo age = dym yo_mama_so_nosy()
ymf(age)
```

**`yo_mama_so_quiet()`**
- Reads string from stdin
- Returns char* (remember to free!)
```yourmom
ymf("Enter your name:")
yo name = dym yo_mama_so_quiet()
ymf(name)
dym yo_mama_so_cheap_free(name)
```

---

### Math Functions

**`yo_mama_so_random()`**
- Returns random integer
- Uses C's `rand()`
```yourmom
yo dice = dym yo_mama_so_random() % 6 + 1
ymf(dice)  // 1-6
```

**`yo_mama_so_round(x)`**
- Round to nearest integer
```yourmom
yo rounded = dym yo_mama_so_round(3.7)
ymf(rounded)  // 4
```

**`yo_mama_so_square(x)`**
- Returns x²
```yourmom
yo sq = dym yo_mama_so_square(5)
ymf(sq)  // 25
```

**`yo_mama_so_thicc(x)`**
- Absolute value |x|
```yourmom
yo abs_val = dym yo_mama_so_thicc(-42)
ymf(abs_val)  // 42
```

**`yo_mama_so_wide(x)`**
- Square root √x
```yourmom
yo root = dym yo_mama_so_wide(16)
ymf(root)  // 4
```

---

### String Functions

**`yo_mama_so_long(s)`**
- String length (strlen)
```yourmom
yo len = dym yo_mama_so_long("hello")
ymf(len)  // 5
```

**`yo_mama_so_cheap_copy(dst, src)`**
- String copy (strcpy)
```yourmom
yo dest = dym yo_mama_so_fat_malloc(100)
dym yo_mama_so_cheap_copy(dest, "Hello")
ymf(dest)
```

**`yo_mama_so_connected(dst, src)`**
- String concatenation (strcat)
```yourmom
yo str = dym yo_mama_so_fat_malloc(100)
dym yo_mama_so_cheap_copy(str, "Hello ")
dym yo_mama_so_connected(str, "World")
ymf(str)  // "Hello World"
```

---

### Memory Functions

**`yo_mama_so_fat_malloc(size)`**
- Allocate memory
- Quantum-aware (works with quantum types)
```yourmom
yo buffer = dym yo_mama_so_fat_malloc(256)
// Use buffer...
dym yo_mama_so_cheap_free(buffer)
```

**`yo_mama_so_cheap_free(ptr)`**
- Free allocated memory
```yourmom
yo data = dym yo_mama_so_fat_malloc(1024)
dym yo_mama_so_cheap_free(data)
```

---

### Quantum Functions

**`heisenberg(vals)`**
- Create Heisenberg variable
- Re-collapses on EVERY read
```yourmom
yo h = heisenberg(1 | 2 | 3 | 4)
ymf(h)  // Might be 2
ymf(h)  // Might be 4
ymf(h)  // Might be 1
```

**`entangled_with var`**
- Create entangled variable
- Always collapses to same value as target
```yourmom
yo x = 10 | 20 | 30
yo y = entangled_with x
ymf(x)  // e.g., 20
ymf(y)  // Also 20, guaranteed
```

**`quantum_sort(vals)`**
- Sort superposition, return minimum
```yourmom
yo sorted = quantum_sort(5 | 3 | 1 | 4 | 2)
// Prints: [1, 2, 3, 4, 5] -> 1
```

**`yo_mama_observe(var)`**
- Force collapse of quantum variable
- Returns collapsed value
```yourmom
yo x = 10 | 20 | 30
yo val = dym yo_mama_observe(x)
ymf(val)
```

**`yo_mama_collapsed(var)`**
- Check if variable already collapsed
- Returns 1 if collapsed, 0 if still in superposition
```yourmom
yo x = 10 | 20 | 30
ymf(dym yo_mama_collapsed(x))  // 0
ymf(x)  // Collapses
ymf(dym yo_mama_collapsed(x))  // 1
```

---

### Quantum Number Types

These aren't functions, they're quantum type keywords:

**`int_all`**
- Any 32-bit integer
- Range: -2,147,483,648 to 2,147,483,647
```yourmom
yo x = int_all
ymf(x)  // Random int like 1847293
```

**`real_all`**
- Any double-precision float
- Truly random double
```yourmom
yo x = real_all
ymf(x)  // e.g., 3.14159265358979
```

**`rational_all`**
- Random fraction (fully reduced)
- Always p/q where gcd(p,q) = 1
```yourmom
yo x = rational_all
ymf(x)  // e.g., 355/113
```

**`irrational_all`**
- Famous irrational constant
- π, e, √2, √3, √5, φ, ln(2), γ, ζ(3), etc.
```yourmom
yo x = irrational_all
ymf(x)  // e.g., "pi ~= 3.14159265358979"
```

**`imaginary_all`**
- Complex number (a+bi, b≠0)
```yourmom
yo x = imaginary_all
ymf(x)  // e.g., "3.5+2.1i"
```

**`quaternion_all`**
- Quaternion (w+xi+yj+zk)
- 4D rotation representation
```yourmom
yo q = quaternion_all
ymf(q)  // e.g., "1.2+3.4i+5.6j-7.8k"
```

**`transfinite_all`**
- Transfinite cardinal/ordinal
- ℵ₀, ℵ₁, ℵ₂, ω, ω+1, ω², ω^ω, ε₀
```yourmom
yo t = transfinite_all
ymf(t)  // e.g., "omega^omega"
```

---

## qol.momjoke

**"Quality of Life Library"**

Load this alongside jksmpl.momjoke for enhanced features.

### How to Enable

In your `.yourdad` file:
```yourdad
daddy_says_wear_these_hats {
    "jksmpl.momjoke"
    "qol.momjoke"
}
```

---

### Formatting Functions

**`ymff` → `printf`**
- Formatted print (no auto newline)
- Use printf-style format strings
```yourmom
yo x = 42
yo y = 3.14
ymff("x is %d and y is %.2f\n", x, y)
// Output: x is 42 and y is 3.14
```

**`ymffl` → `puts`**
- Print string with newline
```yourmom
ymffl("Hello world")
```

**`ymerr` → `fprintf`**
- Print to stderr
```yourmom
ymerr(stderr, "Error: %s\n", "Something broke")
```

---

### Time/Sleep Functions

**`ymsleep` / `ymwait` → `yo_mama_so_slow`**
- Sleep N milliseconds
```yourmom
ymf("Waiting 2 seconds...")
ymsleep(2000)
ymf("Done!")
```

---

### Random Functions

**`ymrng` → `yo_mama_so_random_between`**
- Random int between min and max (inclusive)
```yourmom
yo dice = dym ymrng(1, 6)
ymf(dice)  // 1-6
```

**`ymrand` → `yo_mama_so_random`**
- Random int (no range)
- Alias for existing function
```yourmom
yo random = dym ymrand()
```

---

### File I/O Functions

**`ymrf` → `yo_mama_so_nosy_about_files`**
- Read entire file to string
- Returns char* (MUST free after use!)
```yourmom
yo content = dym ymrf("data.txt")
mama_said content != daddy_left {
    ymf(content)
    ymfree(content)
} mama_lied {
    ymf("File not found")
}
```

**`ymwf` → `yo_mama_so_chatty_about_files`**
- Write string to file
- Returns 1 on success, 0 on failure
```yourmom
yo success = dym ymwf("out.txt", "Hello from .yourmom!\n")
mama_said success == 1 {
    ymf("File written!")
}
```

---

### String Utilities

**`ymfound` → `yo_mama_so_found`**
- Check if string contains substring
- Returns 1 if found, 0 if not
```yourmom
yo text = "hello world"
mama_said dym ymfound(text, "world") == 1 {
    ymf("Found it!")
}
```

**`ymupper` → `yo_mama_so_upper`**
- Convert string to uppercase (in-place)
```yourmom
yo str = dym ymalloc(100)
ymcpy(str, "hello")
dym ymupper(str)
ymf(str)  // "HELLO"
```

**`ymlower` → `yo_mama_so_lower`**
- Convert string to lowercase (in-place)
```yourmom
yo str = dym ymalloc(100)
ymcpy(str, "HELLO")
dym ymlower(str)
ymf(str)  // "hello"
```

**`ymlen` → `yo_mama_so_long`**
- String length (shorter alias)
```yourmom
yo len = dym ymlen("hello")
ymf(len)  // 5
```

**`ymcpy` → `yo_mama_so_cheap_copy`**
- String copy (shorter alias)
```yourmom
yo dest = dym ymalloc(100)
ymcpy(dest, "Hello")
```

**`ymcat` → `yo_mama_so_connected`**
- String concatenation (shorter alias)
```yourmom
yo str = dym ymalloc(100)
ymcpy(str, "Hello ")
ymcat(str, "World")
```

---

### Math Shortcuts

**`ymabs` → `yo_mama_so_thicc`**
```yourmom
yo abs_val = dym ymabs(-42)
```

**`ymsqrt` → `yo_mama_so_wide`**
```yourmom
yo root = dym ymsqrt(16)
```

**`ymsq` → `yo_mama_so_square`**
```yourmom
yo sq = dym ymsq(5)
```

**`ymround` → `yo_mama_so_round`**
```yourmom
yo rounded = dym ymround(3.7)
```

---

### Memory Shortcuts

**`ymalloc` → `yo_mama_so_fat_malloc`**
```yourmom
yo buffer = dym ymalloc(256)
```

**`ymfree` → `yo_mama_so_cheap_free`**
```yourmom
ymfree(buffer)
```

---

### Stress Testing

**`ymstress` → `yo_mama_so_stressed`**
- Benchmark quantum collapse performance
- Measures how fast quantum variables collapse
```yourmom
dym ymstress()
// Runs performance test and prints results
```

---

## quantum_runtime.h

**The quantum mechanics engine that makes everything work**

This is auto-included by the compiler. You don't need to explicitly import it.

### Internal Types

#### `quantum_int`
```c
typedef struct {
    int* values;       // Array of possible values
    int count;         // Number of possibilities
    int collapsed_idx; // -1 if superposition, else index
    int collapsed_val; // Cached collapsed value
} quantum_int;
```

**How it works:**
1. Created with multiple values → `collapsed_idx = -1`
2. First read triggers collapse → picks random index
3. Caches result in `collapsed_val`
4. All future reads return cached value

---

### Core Runtime Functions

These are internal C functions that power the language:

**`_quantum_init_rng()`**
- Initialize random number generator
- Seeds with `time(NULL)`
- Called automatically on first quantum operation

**`_quantum_create(int val)`**
- Create quantum_int from single value
- Used for classical → quantum conversion
```c
quantum_int x = _quantum_create(42);
```

**`_quantum_superposition(int* vals, int count)`**
- Create quantum_int from multiple values
- Powers the `|` operator
```c
int vals[] = {10, 20, 30};
quantum_int x = _quantum_superposition(vals, 3);
```

**`_quantum_observe(quantum_int* q)`**
- Force collapse and return value
- Called on first read
- Powers `yo_mama_observe()`
```c
int val = _quantum_observe(&x);
```

**`_quantum_is_collapsed(quantum_int* q)`**
- Check if already collapsed
- Returns 1 if collapsed, 0 if not
- Powers `yo_mama_collapsed()`
```c
if (_quantum_is_collapsed(&x)) {
    printf("Already collapsed\n");
}
```

**`_quantum_get(quantum_int* q)`**
- Get value (same as observe)
```c
int val = _quantum_get(&x);
```

---

### Quantum Arithmetic

All arithmetic forces observation first, then operates on collapsed values:

**`_quantum_add(a, b)`** → a + b (both collapsed)  
**`_quantum_sub(a, b)`** → a - b  
**`_quantum_mul(a, b)`** → a × b  
**`_quantum_div(a, b)`** → a ÷ b  

**`_quantum_eq(a, b)`** → a == b  
**`_quantum_lt(a, b)`** → a < b  
**`_quantum_gt(a, b)`** → a > b  

---

### Helper Macros

**`SUPERPOSITION(...)`**
- Macro for creating superposition from literals
```c
quantum_int x = SUPERPOSITION(10, 20, 30);
// Expands to:
// int _vals[] = {10, 20, 30};
// _quantum_superposition(_vals, 3);
```

**`_qint(val)`**
- Wrap int as quantum_int
```c
quantum_int x = _qint(42);
```

---

### Error Handling

**`_yo_mama_so_broken(int sig)`**
- Signal handler for crashes
- Prints random your mom joke
- Handles: SIGSEGV, SIGABRT, SIGFPE

**Error jokes:**
- "Yo mama so fat, she caused a stack overflow"
- "Yo mama so dumb, she dereferenced a NULL pointer"
- "Yo mama so ugly, even the CPU refused to execute"
- "Yo mama so slow, she caused a heap corruption"
- "Yo mama so cheap, she freed memory twice"
- "Yo mama so fat, she exceeded the address space"
- "Yo mama so random, she corrupted the RNG seed"

**`_quantum_init_errors()`**
- Install error handlers
- Called at program start
```c
int main() {
    _quantum_init_errors();
    _quantum_init_rng();
    // Your code...
}
```

---

## Quick Reference

### Most Common Operations

```yourmom
// Print
ymf("Hello")                    // Print string
ymf(42)                         // Print number
yo_mama_so_loud("YELLING")      // Uppercase print

// Input
yo num = dym yo_mama_so_nosy()  // Read int
yo str = dym yo_mama_so_quiet() // Read string

// Quantum
yo x = 10 | 20 | 30             // Superposition
yo h = heisenberg(1 | 2 | 3)    // Heisenberg
yo e = entangled_with x         // Entangled

// Math
yo rand = dym ymrng(1, 100)     // Random 1-100
yo abs = dym ymabs(-5)          // Absolute value
yo sqrt = dym ymsqrt(16)        // Square root

// Files
yo content = dym ymrf("file.txt")   // Read file
dym ymwf("out.txt", "data")         // Write file

// Strings
yo len = dym ymlen(str)             // Length
mama_said dym ymfound(str, "test") == 1  // Contains

// Memory
yo buf = dym ymalloc(256)       // Allocate
ymfree(buf)                     // Free

// Time
ymsleep(1000)                   // Sleep 1 second
```

---

## Usage Examples

### Complete Programs

#### Example 1: Number Guessing Game

```yourmom
yo mama_main() {
    // Secret number using quantum
    yo secret = heisenberg(1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10)
    yo target = 0
    
    // Collapse to target
    mama_said_maybe {
        target = 5
    } mama_lied {
        target = 8
    }
    
    yo_mama_so_loud("GUESS THE NUMBER (1-10)")
    
    yo guess = dym yo_mama_so_nosy()
    
    mama_said guess == target {
        yo_mama_so_loud("YOU WIN!")
    } mama_lied {
        ymf("Wrong! It was:")
        ymf(target)
    }
}
```

#### Example 2: File Processing

```yourmom
yo mama_main() {
    // Read file
    yo content = dym ymrf("input.txt")
    
    mama_said content == daddy_left {
        ymerr(stderr, "Error: File not found\n")
        ret 1
    }
    
    // Convert to uppercase
    dym ymupper(content)
    
    // Write to output
    yo success = dym ymwf("output.txt", content)
    
    mama_said success == 1 {
        ymf("File processed successfully!")
    } mama_lied {
        ymf("Failed to write output")
    }
    
    // Clean up
    ymfree(content)
    ret 0
}
```

#### Example 3: Quantum Dice Roller

```yourmom
yo roll_dice() {
    yo dice = dym ymrng(1, 6)
    ret dice
}

yo mama_main() {
    ymf("Rolling 3 dice...")
    ymsleep(500)
    
    yo d1 = dym roll_dice()
    yo d2 = dym roll_dice()
    yo d3 = dym roll_dice()
    
    ymff("Dice: %d, %d, %d\n", d1, d2, d3)
    
    yo total = d1 + d2 + d3
    ymff("Total: %d\n", total)
    
    mama_said total >= 15 {
        yo_mama_so_loud("HIGH ROLL!")
    }
}
```

#### Example 4: All Number Types Demo

```yourmom
yo mama_main() {
    ymf("=== ALL QUANTUM NUMBER TYPES ===")
    ymf("")
    
    // Regular superposition
    ymf("Superposition:")
    yo s = 10 | 20 | 30 | 40
    ymf(s)
    ymf("")
    
    // All number types
    ymf("int_all:")
    yo i = int_all
    ymf(i)
    
    ymf("real_all:")
    yo r = real_all
    ymf(r)
    
    ymf("rational_all:")
    yo rat = rational_all
    ymf(rat)
    
    ymf("irrational_all:")
    yo irr = irrational_all
    ymf(irr)
    
    ymf("imaginary_all:")
    yo img = imaginary_all
    ymf(img)
    
    ymf("quaternion_all:")
    yo q = quaternion_all
    ymf(q)
    
    ymf("transfinite_all:")
    yo t = transfinite_all
    ymf(t)
    
    ymf("")
    ymf("=== HEISENBERG TEST ===")
    yo h = heisenberg(1 | 2 | 3 | 4 | 5)
    ymf(h)  // Different
    ymf(h)  // Every
    ymf(h)  // Time
}
```

---

## Library Comparison Table

| Feature | jksmpl.momjoke | qol.momjoke | quantum_runtime.h |
|---------|----------------|-------------|-------------------|
| Core shortcuts | ✅ | ❌ | ❌ |
| Basic I/O | ✅ | ❌ | ❌ |
| Formatted print | ❌ | ✅ | ❌ |
| File I/O | ❌ | ✅ | ❌ |
| String utilities | ✅ Basic | ✅ Extended | ❌ |
| Math functions | ✅ Basic | ✅ Aliases | ❌ |
| Time/sleep | ❌ | ✅ | ❌ |
| Quantum mechanics | ✅ Keywords | ❌ | ✅ Engine |
| Memory management | ✅ | ✅ Aliases | ❌ |
| Error handling | ❌ | ❌ | ✅ |
| Auto-loaded | ✅ | ❌ | ✅ |

---

## Best Practices

### When to Use Each Library

**Always use jksmpl.momjoke:**
- It's auto-loaded by default
- Provides essential shortcuts
- Required for basic programs

**Use qol.momjoke when you need:**
- File I/O operations
- Formatted printing (printf-style)
- String searching/manipulation
- Sleep/timing functions
- Shorter aliases for common operations

**quantum_runtime.h is automatic:**
- Always included by compiler
- No need to think about it
- Just use quantum features naturally

### Memory Management Tips

```yourmom
// ALWAYS free what you allocate
yo buffer = dym ymalloc(256)
// ... use buffer ...
ymfree(buffer)

// ALWAYS free file reads
yo content = dym ymrf("file.txt")
// ... use content ...
ymfree(content)

// DON'T free string literals
ymf("Hello")  // This is fine, no malloc

// DON'T free yo_mama_so_quiet() result if you didn't use it
```

---

## Performance Notes

### Quantum Variable Overhead

- **First read:** ~1μs (collapse + cache)
- **Subsequent reads:** ~0ns (cached)
- **Heisenberg reads:** ~1μs per read (no cache)

### Function Call Overhead

- Shortcuts are zero-cost (compile-time expansion)
- `dym func()` is just a regular C function call
- No runtime penalty for using shortcuts

---

## Troubleshooting

### Common Issues

**Q: "undefined reference to yo_mama_so_fat"**  
A: Make sure jksmpl.momjoke is loaded (it should be by default)

**Q: "undefined reference to ymrf"**  
A: Add qol.momjoke to your .yourdad file

**Q: Quantum variable always returns same value**  
A: That's correct! Once collapsed, it stays collapsed. Use `heisenberg()` if you want re-randomization.

**Q: Segfault with cryptic your mom joke**  
A: Check your pointer usage. The joke is just flavor - the real error is a segfault.

---

**This is your complete library reference. Keep it handy!**

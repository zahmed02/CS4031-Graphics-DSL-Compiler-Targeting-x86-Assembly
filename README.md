Here's a complete **README.md** you can use for your GitHub repository. Copy the content below and save it as `README.md` in your project root (`D:\DevProjects\Rust\CC-Project`), then commit and push.

```markdown
# PixelLang Compiler – A Graphics DSL for x86 Assembly (Irvine32)

**Course:** CS4031 – Compiler Construction  
**Author:** Zubair Ahmed  
**Language:** Rust (latest stable edition)  
**Target:** x86 assembly (MASM) + Irvine32 library → standalone `.exe`

---

## 📖 About

PixelLang is a small educational DSL for drawing 2D graphics on the console.  
It provides variables, arithmetic, comparisons, loops, conditionals, and drawing primitives.  
The compiler translates PixelLang source code into x86 assembly that can be assembled, linked, and run using the Irvine32 library.

---

## 🚀 Features

- Variables (auto‑declared on first use)
- Integer arithmetic: `+`, `-`, `*`, `/` (with negative numbers via unary `-`)
- Comparisons: `<` and `>`
- Conditionals: `if expr then ... end` (no `else` yet)
- Counted loops: `loop expr times ... end`
- Drawing: `draw_pixel x, y, color`, `draw_rect x, y, w, h, color`, `clear_screen`
- Timing: `delay ms`
- Keyboard input: `wait_key`
- Comments: `//` line comment

---

## 🛠️ Compiler Phases (All Covered)

| Phase | Implementation | Rust Module |
|-------|----------------|--------------|
| Lexical analysis | `logos` lexer → token stream | `src/lexer/mod.rs` |
| Syntax analysis | Recursive‑descent parser → AST | `src/parser/mod.rs` |
| Semantic analysis | Symbol table + type checking (auto‑declare) | `src/semantic/mod.rs` |
| Intermediate representation | Three‑address code (TAC) | `src/ir/mod.rs` |
| Machine‑independent optimization | Constant folding (optional with `-O`) | `src/optimizer.rs` |
| Code generation | x86 assembly with Irvine32 | `src/codegen/mod.rs` |
| Assembly + linking | MASM (`ml`) + `link` (external) | (manual) |

---

## 📦 Dependencies & Installation

### Rust Dependencies (automatically managed by Cargo)

All dependencies are declared in `Cargo.toml` and fetched when you run `cargo build`.

| Dependency | Version | Purpose |
|------------|---------|---------|
| `logos` | 0.15 | Lexical analysis (token generation) |
| `clap` | 4.5 | Command‑line argument parsing (`--input`, `--output`, `--optimize`) |
| `anyhow` | 1.0 | Error handling |
| `thiserror` | 2.0 | Custom error types |
| `walkdir` | 2.0 | Utility for directory walking (not heavily used) |
| `colored` | 2.2 | Optional coloured console output |

**Installation steps** (inside the project folder):
```bash
cargo build
```
All dependencies are downloaded from [crates.io](https://crates.io) and stored locally in `target/`.

### External Tools (required for the final executable)

- **MASM (Microsoft Macro Assembler)**: `ml.exe` – used to assemble the generated `.asm` file.
- **Irvine32 library**: A 32‑bit library for console graphics and I/O.  
  It must be placed in a known directory (e.g., `D:\DevTools\Irvine`).
- **Linker**: `link.exe` (part of MASM or Visual Studio).

The commands below assume the Irvine32 files are located at `D:\DevTools\Irvine`.  
Adjust the path if necessary.

---

## 📁 Project Structure

```
D:\DevProjects\Rust\CC-Project
│   .gitignore
│   Cargo.lock
│   Cargo.toml
│   README.md
│
├───asm_output               # Generated .asm, .obj, .exe files (examples)
├───examples                 # Sample .pixel source programs
├───src
│   │   ast.rs               # Abstract Syntax Tree definitions
│   │   main.rs              # Compiler driver (CLI)
│   │   optimizer.rs         # Constant folding
│   │
│   ├───codegen/mod.rs       # x86 code generator
│   ├───ir/mod.rs            # Three‑address code IR
│   ├───lexer/mod.rs         # Logos lexer
│   ├───parser/mod.rs        # Recursive‑descent parser
│   └───semantic/mod.rs      # Symbol table & type checking
└───target                   # Rust build artifacts (ignored by git)
```

---

## 🧪 Compiling and Running a PixelLang Program

### Step 1: Compile the compiler (Rust → executable)
```bash
cd D:\DevProjects\Rust\CC-Project
cargo build
```

### Step 2: Run the compiler on a `.pixel` source file
```bash
cargo run -- --input examples/demo.pixel --output asm_output\demo.asm
```
(Use `--optimize` or `-O` for constant folding)

### Step 3: Assemble the generated `.asm` file (MASM)
```bash
cd asm_output
ml /c /coff /I D:\DevTools\Irvine demo.asm
```

### Step 4: Link the object file into an executable
```bash
link demo.obj D:\DevTools\Irvine\Irvine32.lib kernel32.lib user32.lib /subsystem:console
```

### Step 5: Run the executable
```bash
.\demo.exe
```

**Example for a bouncing rectangle (`bounce.pixel`):**
```bash
cargo run -- --input examples\bounce.pixel --output asm_output\bounce.asm
cd asm_output
ml /c /coff /I D:\DevTools\Irvine bounce.asm
link bounce.obj D:\DevTools\Irvine\Irvine32.lib kernel32.lib user32.lib /subsystem:console
.\bounce.exe
```

---

## 📝 Sample PixelLang Program

### `examples/demo.pixel`
```pixel
x = 10
y = 20
clear_screen
draw_rect x, y, 5, 5, 15
wait_key
```

### `examples/bounce.pixel`
```pixel
x = 10
dx = 1

loop 50 times
    clear_screen
    draw_rect x, 10, 4, 4, 15
    x = x + dx
    if x > 30 then
        dx = -1
    end
    if x < 2 then
        dx = 1
    end
    delay 50
end
wait_key
```

---

## 🧪 All Example Programs (in `examples/`)

| File | Description |
|------|-------------|
| `demo.pixel` | Simple rectangle at fixed position |
| `bounce.pixel` | Moving rectangle that bounces off walls |
| `ball.pixel` | Single pixel moving diagonally and bouncing |
| `diagonal.pixel` | Draws a diagonal line of `*` |
| `grow.pixel` | Rectangle that grows and shrinks |
| `timer.pixel` | Rectangle bar that shrinks like a countdown |
| `checkerboard.pixel` | Diagonal pattern (uses comparisons) |

All generate working `.exe` files.

---

## ⚠️ Limitations (Current Version)

- No `else` clause
- No equality operators (`==`, `!=`, `<=`, `>=`)
- No modulo (`%`)
- No floating‑point numbers
- No user‑defined functions or arrays
- Colors are ignored (always draw `*` or `#` in console)

---

## 📚 How It Works – Compiler Internals

1. **Lexer** (`logos`): Converts source text into tokens (`IDENT`, `INTEGER`, `KW_IF`, ...).
2. **Parser** (hand‑written recursive descent): Builds an AST (abstract syntax tree).
3. **Semantic Analysis**:  
   - Symbol table (variables stored in `.data` section).  
   - Type checking (all integers).  
   - Auto‑declaration of variables on first assignment.
4. **Intermediate Representation (TAC)**: Three‑address code like `t0 = x + y`, `draw_rect t1, t2, ...`.
5. **Optimizer (optional)**: Constant folding (e.g., `5 * 2` → `10`).
6. **Code Generator**: Translates TAC into x86 assembly using Irvine32 conventions.  
   - Variables are stored as `DWORD` in `.data` (prefixed with `_` to avoid MASM reserved names).  
   - Temporaries `t0`, `t1` also live in `.data`.  
   - Drawing helpers are implemented directly in the generated `.asm` (real code, not stubs).
7. **Assembly + Linking**: External steps using `ml` and `link` produce the final `.exe`.

---

## 🔧 Troubleshooting

| Problem | Possible Solution |
|---------|------------------|
| `'ml' is not recognized` | Install MASM or Visual Studio Build Tools with C++ support, or adjust PATH. |
| `Irvine32.inc not found` | Ensure `Irvine32.inc` and `Irvine32.lib` are in `D:\DevTools\Irvine` (or change the `/I` path). |
| `link: fatal error LNK1181` | The object file was not created; check MASM errors. |
| `cargo build` fails with linker errors | Run `rustup default stable-gnu` to use the GNU toolchain (MinGW‑w64) instead of MSVC. |
| Generated assembly has undefined `DrawPixel_helper` | The helper is included at the end of the `.asm` file (since the final code generation). |

---

## 📄 License

This project is for educational purposes as part of CS4031 Compiler Construction.

---

## 🙏 Acknowledgements

- Irvine32 library by Kip Irvine
- Rust crates: `logos`, `clap`, `anyhow`, etc.
- MinGW‑w64 / MSVC toolchains

```

---

## Next Steps

1. **Save the content above** as `README.md` in your project root.
2. **Add, commit, and push**:

```bash
git add README.md
git commit -m "Add comprehensive README with compiler documentation"
git push origin main
```

3. **Refresh your GitHub repository** – you'll see the formatted README on the main page.

Let me know if you want me to adjust any part (e.g., paths, toolchain instructions, or add more examples).

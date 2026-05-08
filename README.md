# CS4031 Graphics DSL Compiler Targeting x86 Assembly

## Course: CS4031 Compiler Construction
## Instructor & Project Supervisor: Miss Shahar Bano Husnine

## Overview
**Language:** Rust (latest stable)  
**Target:** x86 assembly + Irvine32 library → executable console graphics

A small domain‑specific language for drawing 2D graphics and simple animations on the Windows console. The compiler performs all classical compilation phases (lexical, syntax, semantic analysis, intermediate code generation, optimization, and x86 code generation) and produces assembly files that can be assembled with MASM and linked with the Irvine32 library.

## Language Features

| Feature Category | Syntax / Description |
|-----------------|----------------------|
| **Variables** | Auto‑declared on first assignment, integer only |
| **Arithmetic** | `+`, `-`, `*`, `/` (integer division), parentheses, unary minus |
| **Comparisons** | `<` and `>` (results in 0 or 1) |
| **Control Flow** | `if … then … end`, `loop <expr> times … end` |
| **Drawing** | `clear_screen`, `draw_pixel x, y, color`, `draw_rect x, y, w, h, color` |
| **Timing & Input** | `delay ms`, `wait_key` |
| **Comments** | Line comments with `//` |

The generated assembly uses the Irvine32 library for console manipulation (`Gotoxy`, `WriteChar`, `ClrScr`, `Delay`, `ReadChar`).

## Technologies Used

| Category | Technology | Purpose |
|----------|------------|---------|
| **Language** | Rust (latest stable) | Compiler implementation |
| **Lexer** | `logos` 0.15 | Token generation |
| **Parser** | Hand‑written recursive descent | No external parser generator |
| **CLI parsing** | `clap` 4.5 (derive feature) | Command‑line arguments |
| **Error handling** | `anyhow` 1.0 | Flexible error propagation |
| **Assembly target** | x86 (MASM syntax) + Irvine32 library | Executable console graphics |
| **Assembler / Linker** | MASM (`ml.exe`), Windows `link.exe` | Produce `.exe` |

## Prerequisites

### Rust toolchain
- Install Rust from [rustup.rs](https://rustup.rs/) (MSVC toolchain recommended on Windows)  
- Verify: `rustc --version`

### Assembler & linker (MASM + Irvine32)
- Microsoft Macro Assembler (`ml.exe`): comes with Visual Studio Build Tools or Visual Studio 2022 (select “Desktop development with C++”)  
- Irvine32 library: place it, e.g., in `D:\DevTools\Irvine\` (adjust the path in commands)  
- The following files must be present: `Irvine32.inc`, `Irvine32.lib`

### Environment (for this project)
- The compiler outputs `.asm` files; you assemble & link them manually using the provided commands.

## Rust Dependencies (Cargo.toml)

All dependencies are downloaded automatically by Cargo. No manual installation is required.

| Crate | Purpose |
|-------|---------|
| `logos = "0.15"` | Lexical analysis (token generation) |
| `clap = { version = "4.5", features = ["derive"] }` | Command‑line argument parsing |
| `anyhow = "1.0"` | Error handling |
| (no other runtime crates – parser is hand‑written) | |

> **Note:** The compiler uses a **hand‑written recursive‑descent parser** (no `pest` or `nom` in the final version) to keep dependencies minimal and maximise control.

## Compiler Phases Complete Pipeline

The compiler follows the standard structure taught in CS4031:

| Phase | Implementation | Location |
|-------|----------------|----------|
| **Lexical analysis** | `logos`: converts source char stream into tokens | `src/lexer/mod.rs` |
| **Syntax analysis** | Recursive‑descent: builds an AST | `src/parser/mod.rs` |
| **Semantic analysis** | Symbol table + type checking (all integers) | `src/semantic/mod.rs` |
| **Intermediate code generation** | Three‑address code (TAC) with temporaries and labels | `src/ir/mod.rs` |
| **Machine‑independent optimization** | Constant folding (enabled with `-O` flag) | `src/optimizer.rs` |
| **Code generation** | x86 assembly (MASM syntax) + Irvine32 calls | `src/codegen/mod.rs` |
| **Assembly & linking** | External: `ml` (MASM) + `link` | manual commands |

The output assembly is human‑readable, uses `.data` for variables, and includes helper functions (`DrawPixel_helper`, `DrawRect_helper`) that draw `*` and `#` using `Gotoxy`/`WriteChar`.

## Constructing the Compiler

```bash
git clone https://github.com/zahmed02/CS4031-Graphics-DSL-Compiler-Targeting-x86-Assembly.git
cd CS4031-Graphics-DSL-Compiler-Targeting-x86-Assembly
cargo build --release
```
The executable will be at `target/release/cc-project.exe` (or `target/debug/cc-project.exe` for debug builds).

## 📖 Usage

### 1. Compile a `.pixel` source file to assembly

```bash
cargo build
cargo run -- --input examples/demo.pixel --output asm_output/demo.asm
```

Optional constant folding (optimisation):

```bash
cargo run -- --input examples/bounce.pixel --output asm_output/bounce.asm --optimize
```

### 2. Assemble with MASM (example for `demo.asm`)

```bash
cd asm_output
ml /c /coff /I D:\DevTools\Irvine demo.asm
```

### 3. Link with Irvine32 library

```bash
link demo.obj D:\DevTools\Irvine\Irvine32.lib kernel32.lib user32.lib /subsystem:console
```

### 4. Run the executable

```bash
.\demo.exe
```

> **Note:** Adjust the path `/I D:\DevTools\Irvine` and the library paths to where you placed the Irvine32 files.

## Example Programs

All examples are located in the `examples/` folder.

| File | Description |
|------|-------------|
| `demo.pixel` | Draws a small rectangle, waits for a key |
| `bounce.pixel` | A rectangle bouncing left‑right |
| `ball.pixel` | A single pixel bouncing inside a box |
| `diagonal.pixel` | Draws a diagonal line of stars |
| `grow.pixel` | A rectangle that grows and shrinks |
| `checkerboard.pixel` | Draws a checkerboard‑like pattern |
| `timer.pixel` | A shrinking progress bar |

### Sample test with `bounce.pixel`

```bash
cargo run -- --input examples/bounce.pixel --output asm_output/bounce.asm
cd asm_output
ml /c /coff /I D:\DevTools\Irvine bounce.asm
link bounce.obj D:\DevTools\Irvine\Irvine32.lib kernel32.lib user32.lib /subsystem:console
.\bounce.exe
```

## Project Structure

```
.
├── .gitignore
├── Cargo.toml
├── Cargo.lock
├── README.md
├── examples/
│   ├── ball.pixel
│   ├── bounce.pixel
│   ├── checkerboard.pixel
│   ├── demo.pixel
│   ├── diagonal.pixel
│   ├── grow.pixel
│   └── timer.pixel
├── asm_output/
│   └── (generated .asm, .obj, .exe files)
├── src/
│   ├── main.rs
│   ├── ast.rs
│   ├── optimizer.rs
│   ├── lexer/mod.rs
│   ├── parser/mod.rs
│   ├── semantic/mod.rs
│   ├── ir/mod.rs
│   └── codegen/mod.rs
└── target/               (build artefacts, ignored by Git)
```

## Dependencies & Where They Are Used

### Rust crates (declared in `Cargo.toml`)
- **`logos`**: used only in the lexer to generate tokens from source text.  
- **`clap`**: used in `main.rs` to parse command‑line arguments (`--input`, `--output`, `--optimize`).  
- **`anyhow`**: used for flexible error handling throughout the compiler.

All crates are downloaded and compiled into the `target/` directory when you run `cargo build`. No system‑wide installation is required.

### External tools (must be installed separately)
- **MASM (`ml.exe`)**: part of Visual Studio Build Tools or Visual Studio.  
- **Irvine32 library**: available from Kip Irvine’s website. Place it in a known directory (e.g., `D:\DevTools\Irvine`).  
- **Windows SDK linker (`link.exe`)**: also included with Visual Studio Build Tools.

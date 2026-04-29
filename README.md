# PixelLang Compiler – A Graphics DSL Targeting x86 Assembly

**Course:** CS4031 Compiler Construction  
**Language:** Rust (latest stable)  
**Target:** x86 assembly + Irvine32 library → executable console graphics

PixelLang is a small domain‑specific language for drawing 2D graphics and simple animations on the Windows console. The compiler performs all classical compilation phases (lexical, syntax, semantic analysis, intermediate code generation, optimization, and x86 code generation) and produces assembly files that can be assembled with MASM and linked with the Irvine32 library.

---

## ✨ Language Features

- **Variables** – auto‑declared on first assignment, integer only  
- **Arithmetic** – `+`, `-`, `*`, `/` (integer division), parentheses, unary minus  
- **Comparisons** – `<` and `>` (results in 0 or 1)  
- **Control flow** – `if … then … end`, `loop <expr> times … end`  
- **Drawing** – `clear_screen`, `draw_pixel x, y, color`, `draw_rect x, y, w, h, color`  
- **Timing & input** – `delay ms`, `wait_key`  
- **Comments** – line comments with `//`

The generated assembly uses the Irvine32 library for console manipulation (`Gotoxy`, `WriteChar`, `ClrScr`, `Delay`, `ReadChar`).

---

## 🛠️ Prerequisites

### Rust toolchain
- Install Rust from [rustup.rs](https://rustup.rs/) (MSVC toolchain recommended on Windows)  
- Verify: `rustc --version`

### Assembler & linker (MASM + Irvine32)
- Microsoft Macro Assembler (`ml.exe`) – comes with Visual Studio Build Tools or Visual Studio 2022 (select “Desktop development with C++”)  
- Irvine32 library – place it, e.g., in `D:\DevTools\Irvine\` (adjust the path in commands)  
- The following files must be present: `Irvine32.inc`, `Irvine32.lib`

### Environment (for this project)
- The compiler outputs `.asm` files; you assemble & link them manually using the provided commands.

---

## 📦 Rust Dependencies (Cargo.toml)

All dependencies are downloaded automatically by Cargo. No manual installation is required.

| Crate | Purpose |
|-------|---------|
| `logos = "0.15"` | Lexical analysis (token generation) |
| `clap = { version = "4.5", features = ["derive"] }` | Command‑line argument parsing |
| `anyhow = "1.0"` | Error handling |
| (no other runtime crates – parser is hand‑written) | |

> **Note:** The compiler uses a **hand‑written recursive‑descent parser** (no `pest` or `nom` in the final version) to keep dependencies minimal and maximise control.

---

## 🔬 Compiler Phases – Complete Pipeline

The compiler follows the standard structure taught in CS4031:

| Phase | Implementation | Location |
|-------|----------------|----------|
| **Lexical analysis** | `logos` – converts source char stream into tokens | `src/lexer/mod.rs` |
| **Syntax analysis** | Recursive‑descent – builds an AST | `src/parser/mod.rs` |
| **Semantic analysis** | Symbol table + type checking (all integers) | `src/semantic/mod.rs` |
| **Intermediate code generation** | Three‑address code (TAC) with temporaries and labels | `src/ir/mod.rs` |
| **Machine‑independent optimization** | Constant folding (enabled with `-O` flag) | `src/optimizer.rs` |
| **Code generation** | x86 assembly (MASM syntax) + Irvine32 calls | `src/codegen/mod.rs` |
| **Assembly & linking** | External: `ml` (MASM) + `link` | manual commands |

The output assembly is human‑readable, uses `.data` for variables, and includes helper functions (`DrawPixel_helper`, `DrawRect_helper`) that draw `*` and `#` using `Gotoxy`/`WriteChar`.

---

## 🚀 Building the Compiler

```bash
git clone https://github.com/zahmed02/CS4031-Graphics-DSL-Compiler-Targeting-x86-Assembly.git
cd CS4031-Graphics-DSL-Compiler-Targeting-x86-Assembly
cargo build --release

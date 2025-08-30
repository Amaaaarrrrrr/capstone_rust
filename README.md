# Hello Rust - Moringa AI Tutorial 🦀

A beginner-friendly Rust project demonstrating basic setup, compilation, and execution on Ubuntu Linux.

## Project Overview

This is a simple "Hello World" program written in Rust that demonstrates:
- Basic Rust project structure using Cargo
- Simple console output with `println!` macro
- Rust compilation and execution workflow
- Cross-platform development practices

## Prerequisites

- **Operating System**: Ubuntu Linux (tested) or any Unix-like system
- **Internet Connection**: Required for Rust installation
- **Terminal Access**: Basic command line familiarity helpful

## Installation & Setup

### Step 1: Install Rust 🛠

Run the official rustup installer (installs `rustc`, `cargo`, and `rustup`):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When prompted, press **Enter** to proceed with the default installation.

Reload your shell to make commands available:

```bash
source $HOME/.cargo/env
```

### Step 2: Verify Installation ✅

Confirm everything installed correctly:

```bash
rustc --version    # Rust compiler
cargo --version    # Package manager
rustup --version   # Toolchain installer
```

**Expected output format:**
```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
rustup 1.26.0 (5af9b9484 2023-04-05)
```

## Project Setup

### Step 3: Create New Project 🛠

Create and navigate to your Rust project:

```bash
cargo new hello_rust
cd hello_rust
```

**Generated project structure:**
```
hello_rust/
├── Cargo.toml          # Project configuration
├── src/
│   └── main.rs         # Source code
└── .gitignore          # Git ignore rules
```

### Step 4: Customize the Code 🛠

Edit the main source file:

```bash
nano src/main.rs
```

Replace the default content with:

```rust
fn main() {
    println!("Hello, Moringa AI from Rust on Ubuntu! 🦀");
}
```

**Save and exit**: `CTRL+O` → `Enter` → `CTRL+X`

## Running the Project

### Step 5: Build and Run 🛠

Compile and execute in one command:

```bash
cargo run
```

**Expected output:** ✅
```
   Compiling hello_rust v0.1.0 (/home/your-username/hello_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/hello_rust`
Hello, Moringa AI from Rust on Ubuntu! 🦀
```

## Additional Commands

### Development Workflow 🛠

```bash
# Just compile (faster, no execution)
cargo build

# Check code compiles without building binary
cargo check

# Run the compiled binary directly
./target/debug/hello_rust

# Build optimized release version
cargo build --release
./target/release/hello_rust
```

### Useful Development Commands

```bash
cargo fmt          # Format code automatically
cargo clippy       # Lint and get improvement suggestions
cargo test         # Run tests (none in this project yet)
cargo doc --open   # Generate and view documentation
cargo clean        # Remove build artifacts
```

## Project Files Explained

### `Cargo.toml`
```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
# Add external crates here when needed
```

- **`name`**: Project identifier
- **`version`**: Semantic versioning
- **`edition`**: Rust edition (language version)
- **`dependencies`**: External libraries (none needed for Hello World)

### `src/main.rs`
```rust
fn main() {
    println!("Hello, Moringa AI from Rust on Ubuntu! 🦀");
}
```

- **`fn main()`**: Entry point of the program
- **`println!`**: Macro for printing to console with newline
- **`🦀`**: Rust's unofficial mascot emoji (Ferris the crab)

## Troubleshooting

### Common Issues:

**Command not found errors:**
```bash
# If rustc/cargo commands don't work:
source $HOME/.cargo/env
# Or restart your terminal
```

**Permission errors during installation:**
```bash
# Never use sudo with rustup installer
# If you accidentally did, remove and reinstall:
rm -rf ~/.cargo ~/.rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Compilation errors:**
- Read the error messages carefully - Rust's compiler provides excellent guidance
- Most beginner errors are syntax-related (missing semicolons, typos)

## Next Steps

After completing this tutorial, consider:

1. **Read "The Rust Book"**: https://doc.rust-lang.org/book/
2. **Try Rustlings**: https://github.com/rust-lang/rustlings
3. **Explore Rust by Example**: https://doc.rust-lang.org/rust-by-example/
4. **Build a calculator or simple CLI tool**
5. **Join the Rust community**: https://users.rust-lang.org/

## Learning Resources

- **Official Rust Website**: https://www.rust-lang.org/
- **The Rust Programming Language Book**: https://doc.rust-lang.org/book/
- **Rust Standard Library Docs**: https://doc.rust-lang.org/std/
- **Cargo Documentation**: https://doc.rust-lang.org/cargo/
- **Rust Playground**: https://play.rust-lang.org/ (try Rust online)

## Project Information

- **Created**: As part of Moringa AI Capstone Project
- **Purpose**: Demonstrate Rust installation and basic project workflow
- **Target Audience**: Complete Rust beginners
- **Tested On**: Ubuntu Linux
- **Rust Edition**: 2021

## License

This project is for educational purposes. Rust itself is dual-licensed under Apache 2.0 and MIT licenses.

---

**🎉 Congratulations!** You've successfully set up Rust and created your first Rust program. Welcome to the Rust community! 🦀
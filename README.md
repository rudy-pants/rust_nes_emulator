# NES Emulator

A NES emulator written in Rust using [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) / [egui](https://github.com/emilk/egui).

---

## Cross-Compiling from Ubuntu (WSL) to Windows

This project is developed in a WSL Ubuntu environment and can be compiled to a native Windows `.exe` using the `x86_64-pc-windows-gnu` target and the MinGW-w64 toolchain.

### Prerequisites

#### 1. Install the Windows Rust target

```bash
rustup target add x86_64-pc-windows-gnu
```

#### 2. Install the MinGW-w64 cross-compiler

```bash
sudo apt update
sudo apt install gcc-mingw-w64-x86-64
```

This provides the `x86_64-w64-mingw32-gcc` linker that Cargo will use to produce a Windows PE executable.

#### 3. Configure Cargo to use the MinGW linker

Create (or edit) `.cargo/config.toml` in the project root:

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```

### Building

#### Debug build

```bash
cargo build --target x86_64-pc-windows-gnu
```

The output will be at:

```
target/x86_64-pc-windows-gnu/debug/nes_emulator.exe
```

#### Release build

```bash
cargo build --target x86_64-pc-windows-gnu --release
```

The output will be at:

```
target/x86_64-pc-windows-gnu/release/nes_emulator.exe
```

### Suppressing the console window on Windows (optional)

By default, Windows will open a console window alongside the GUI. To suppress it, add this attribute to the top of `src/main.rs`:

```rust
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
```

### Running the executable

Copy the `.exe` to a Windows machine (or your Windows host from WSL) and run it directly — no additional runtime libraries are required. The MinGW-w64 toolchain statically links the necessary C runtime by default.

To copy to your Windows host from WSL:

```bash
cp target/x86_64-pc-windows-gnu/release/nes_emulator.exe /mnt/c/Users/<YourUser>/Desktop/
```

### Troubleshooting

| Problem | Fix |
|---|---|
| `linker 'x86_64-w64-mingw32-gcc' not found` | Run `sudo apt install gcc-mingw-w64-x86-64` |
| `error: could not find target x86_64-pc-windows-gnu` | Run `rustup target add x86_64-pc-windows-gnu` |
| `.cargo/config.toml` not picked up | Make sure the file is in the project root (same directory as `Cargo.toml`), not in `~/.cargo/` |
| Blank/black window on Windows | Ensure your GPU drivers support OpenGL 3.x — eframe uses OpenGL (glow) by default |

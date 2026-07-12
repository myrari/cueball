# `cueball`: An open-source theatrical cue player

This is a piece of software designed to ease and automate the sequenced playing
of various cues, especially sound and video. Designed to be an open-source
alternative to the commerical [QLab](https://qlab.app/), the eventual goal of
this project is to have a fully-featured cue player capable of professional
showrunning, while still being cross-platform and free for all to use.

**This project is stil *very* much under development, and is not yet ready for
real use.**

## Building from source

### Prerequisites

- A Rust toolchain (tested with rustc/cargo 1.96.1)
- Lua 5.4 development files, discoverable via `pkg-config` (needed by the
  `mlua` dependency)

On macOS with Homebrew:

```sh
brew install rust lua@5.4 pkg-config
```

`pkg-config` looks for a package named `lua54`, but Homebrew's `lua@5.4`
installs its `.pc` file as `lua5.4.pc`. If `cargo build` fails with an error
like `The system library 'lua54' required by crate 'mlua-sys' was not
found`, point `pkg-config` at a symlink with the expected name:

```sh
mkdir -p /tmp/lua-pkgconfig
ln -sf "$(brew --prefix lua@5.4)/lib/pkgconfig/lua5.4.pc" /tmp/lua-pkgconfig/lua54.pc
export PKG_CONFIG_PATH="/tmp/lua-pkgconfig:$PKG_CONFIG_PATH"
```

On NixOS/Nix, `shell.nix` in this repo already sets up a working `cargo`/`rustc`/`lua` environment via `nix-shell`.

### Build and run

```sh
cargo build
```

This produces three binaries under `target/debug/`: `cueball` (the GUI
application), `cueball-cli`, and `cueball-lua-testbench`.

To run the GUI application:

```sh
cargo run --bin cueball
```

### Known limitations (as of the instructions above)

- Only tested on macOS (arm64). Not yet verified on Linux or Windows.
- `cueball-cli` is an interactive terminal application (built on `reedline`)
  and requires a real terminal session -- it will not behave usefully when
  run with piped/non-interactive stdin.
- `eframe`'s `wayland` feature is enabled in `Cargo.toml`, which is a
  Linux-specific windowing backend; it did not prevent building or running
  the GUI binary on macOS in testing.

*Build instructions last verified 2026-07-11 against commit `2f1e38c`.*

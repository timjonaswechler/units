# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

**Build and Check:**
- `cargo check` - Quick compilation check
- `cargo build` - Full build
- `cargo build --release` - Optimized build

**Testing:**
- `cargo test` - Run tests (currently 0 unit tests, 65 doc tests with failures)
- `cargo test --doc` - Run documentation tests only
- `cargo test --lib` - Run library unit tests only

**Documentation:**
- `cargo doc` - Generate documentation
- `cargo doc --open` - Generate and open documentation

## Architecture

This is a **standalone Rust crate for type-safe physics units with dimensional analysis**, designed for scientific computing and astronomical calculations. It is NOT a game engine or simulation - it's a pure data library focused on preventing unit conversion errors at compile time.

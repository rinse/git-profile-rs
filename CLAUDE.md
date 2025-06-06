# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

git-profile-rs is a Rust CLI tool for managing Git profiles. It allows users to switch between different Git configurations (user.name, user.email, etc.) stored in profile files located at `$XDG_CONFIG/git-profile/<PROFILE-NAME>.gitconfig`.

## Architecture

- **CLI Layer**: `src/cli.rs` defines the command-line interface using clap with derive macros
- **Main Entry**: `src/main.rs` handles argument parsing and delegates to appropriate modules
- **Profile Module**: `src/profile/` contains profile management functionality
  - `src/profile/switch.rs`: Core switching logic using git2 to modify Git configuration

The tool modifies Git configuration by adding include paths to either local repository config (default) or global Git config (with --global flag).

## Development Commands

- **Build**: `cargo build`
- **Run**: `cargo run -- switch <PROFILE-NAME> [--global]`
- **Check code**: `cargo clippy`
- **Check formatting**: `cargo fmt --check` (run before committing)
- **Run tests**: `cargo test`
- **Test switching**: `cargo run -- switch sample` then verify with `git config user.name` and `git config user.email`

## Important Reminders

- **Always commit Cargo.lock**: When modifying `Cargo.toml`, always run `cargo build` and include `Cargo.lock` in your commit with `git add Cargo.lock`

## Git Workflow Preferences

- **Discard changes**: Use `git restore <PATH>` instead of `git checkout -- <PATH>`
- **Push branches**: Don't use `-u` flag (no upstream tracking) - just `git push origin <branch>`
- **Add files**: Use specific file paths with `git add file1 file2 file3` rather than `git add -A` or `git add .`
- **Tag format**: Use semantic versioning without 'v' prefix (e.g., `1.0.0` not `v1.0.0`)
- **PR creation**: Always collect git context first with `git diff main...HEAD` and `git log` to understand all changes

## Code Style

- No empty lines within function bodies
- All source files must end with a newline character
- Empty lines must contain no spaces or tabs (completely empty)
- Prefer functional-style code with conditional assignment to function pointers over duplicating code blocks
- Use concise conditional logic to avoid verbose if-else structures
- Order functions with dependents before dependencies (callers before callees)
- Place public functions before private functions
- Don't use type aliases for Result types - write `Result<T, ErrorType>` explicitly

## Profile Setup

Profiles are expected at `$XDG_CONFIG_HOME/git-profile/<PROFILE-NAME>.gitconfig` (or `~/.config/git-profile/` if XDG_CONFIG_HOME is not set). Each profile should contain standard Git configuration like:

```gitconfig
[user]
    name = Your Name
    email = your.email@example.com
```
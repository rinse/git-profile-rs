# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

git-profile-rs is a Rust CLI tool for managing Git profiles. It allows users to switch between different Git configurations (user.name, user.email, etc.) stored in profile files located at `$XDG_CONFIG/git-profile/<PROFILE-NAME>.gitconfig`.

## Architecture

- **CLI Layer**: `src/cli.rs` defines the command-line interface using clap with derive macros
- **Main Entry**: `src/main.rs` handles argument parsing and delegates to appropriate modules
- **Profile Module**: `src/profile/` contains profile management functionality
  - `src/profile/switch.rs`: Core switching logic using git2 to modify Git configuration

The tool modifies Git configuration by adding include paths to the local repository config.

## Development Commands

- **Build**: `cargo build`
- **Run**: `cargo run -- switch <PROFILE-NAME>`
- **Check code**: `cargo clippy`
- **Check formatting**: `cargo fmt --check` (run before committing)
- **Run tests**: `cargo test`
- **Test switching**: `cargo run -- switch sample` then verify with `git config user.name` and `git config user.email`

## Important Reminders

- **Always commit Cargo.lock**: When modifying `Cargo.toml`, always run `cargo build` and include `Cargo.lock` in your commit with `git add Cargo.lock`

## Code Style

- No empty lines within function bodies
- Single empty line between function declarations
- All source files must end with a newline character
- Empty lines must contain no spaces or tabs (completely empty)
- Prefer functional-style code with conditional assignment to function pointers over duplicating code blocks
- Use concise conditional logic to avoid verbose if-else structures
- Order functions with dependents before dependencies (callers before callees)
- Place public functions before private functions
- Don't use type aliases for Result types - write `Result<T, ErrorType>` explicitly

## Naming Conventions

- Struct names follow the pattern `{Purpose}{Implementation}` (e.g., `ConfigDirGitProfile`, `GitConfigGit2`)
- File names match struct names in snake_case (e.g., `config_dir_git_profile.rs`, `git_config_git2.rs`)
- This ensures consistency between file names and the primary struct they contain

## Profile Setup

Profiles are expected at `$XDG_CONFIG_HOME/git-profile/<PROFILE-NAME>.gitconfig` (or `~/.config/git-profile/` if XDG_CONFIG_HOME is not set). Each profile should contain standard Git configuration like:

```gitconfig
[user]
    name = Your Name
    email = your.email@example.com
```

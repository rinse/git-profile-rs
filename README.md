# git-profile-rs

A Rust implementation for my Git-Profile.

## Disclaimer

This project is primarily AI-generated and created for testing purposes. The author assumes no responsibility for any issues, data loss, or other problems that may arise from using this tool. Use at your own risk.

## Quick Start

```bash
# 1. Install (download from releases or cargo install git-profile)
# 2. Create a profile
mkdir -p ~/.config/git-profile
echo '[user]
    name = "Your Name"
    email = "your.email@example.com"' > ~/.config/git-profile/work.gitconfig
# 3. Switch to profile in current repo
git-profile switch work
```

## Installation

### Download from GitHub Releases

Download the latest binary for your platform from [GitHub Releases](https://github.com/rinse/git-profile-rs/releases):

- **Linux**: `git-profile-x86_64-unknown-linux-gnu.tar.gz`
- **macOS (Intel)**: `git-profile-x86_64-apple-darwin.tar.gz`
- **macOS (Apple Silicon)**: `git-profile-aarch64-apple-darwin.tar.gz`
- **Windows**: `git-profile-x86_64-pc-windows-msvc.zip`

Extract and move the binary to your PATH:

```bash
# Example for Linux/macOS
tar -xzf git-profile-*.tar.gz
mv git-profile $HOME/.local/bin/
```

### Build from source

If you have Rust installed, you can install directly from cargo:

```bash
cargo install git-profile
```

## Usage

You need to prepare for a *git profile* first.
Your profiles are placed on `$XDG_CONFIG/git-profile/<PROFILE-NAME>.gitconfig`.

The following command will switch the *git profile*.

```bash
git-profile switch <PROFILE-NAME> [--global]
```

### Local vs Global Configuration

- **Without `--global`**: The profile is applied only to the current repository by modifying `.git/config`. This is useful when you want different identities for different projects.
- **With `--global`**: The profile is applied system-wide by modifying your global Git configuration (`~/.gitconfig`). All repositories will use this profile unless overridden by local configuration.

Note: Local repository configuration always takes precedence over global configuration.

This command will write down the following line in the configuration file.
Note that `$XDG_CONFIG` will be resolved on `.git/config` because of the restriction of Git.

```gitconfig
[include]
	path = <$XDG_CONFIG>/git-profile/<PROFILE-NAME>.gitconfig
```

### Verifying Active Profile

To check which profile is currently active:

```bash
# Show all include paths in your Git configuration
git config --list --show-origin | grep include.path

# Check current user configuration
git config user.name
git config user.email
```

To see if a profile is set locally or globally:

```bash
# Check local repository config
git config --local --get-regexp include.path

# Check global config
git config --global --get-regexp include.path
```

## Sample Profile

To create a profile, the filename must have a leading part that matches the profile name you'll use with the `switch` command. For example, to use profile `work`, create a file at `~/.config/git-profile/work.gitconfig`:

```gitconfig
[user]
    name = John Doe
    email = john.doe@example.com
[core]
    editor = vim
```

## Troubleshooting

### Profile not found

If you get an error that the profile cannot be found, check:

1. The profile file exists at the correct path:
   ```bash
   ls -la ~/.config/git-profile/<PROFILE-NAME>.gitconfig
   ```

2. If `XDG_CONFIG_HOME` is set, profiles should be in:
   ```bash
   ls -la $XDG_CONFIG_HOME/git-profile/
   ```

### Changes not taking effect

Git configuration follows a precedence order:

1. Repository-specific config (`.git/config`) - highest priority
2. User-specific config (`~/.gitconfig`)
3. System-wide config (`/etc/gitconfig`) - lowest priority

If your profile changes aren't working:
- Check if there are conflicting values in `.git/config`
- Use `git config --list --show-origin` to see where each setting comes from

### XDG_CONFIG_HOME not set

If `XDG_CONFIG_HOME` is not set, git-profile defaults to `~/.config`. You can set it explicitly:

```bash
export XDG_CONFIG_HOME="$HOME/.config"
```

### Permission denied

If you get permission errors when installing to `~/.local/bin`:

```bash
# Create the directory if it doesn't exist
mkdir -p $HOME/.local/bin

# Ensure it's in your PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```


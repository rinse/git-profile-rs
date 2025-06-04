# git-profile-rs

A Rust implementation for my Git-Profile.

## Usage

You need to prepare for a *git profile* first.
Your profiles are placed on `$XDG_CONFIG/git-profile/<PROFILE-NAME>.gitconfig`.

The following command will switch the *git profile*.

```bash
git-profile switch <PROFILE-NAME> [--global]
```

This command will write down the following line in the local repository.
Note that `$XDG_CONFIG` will be resolved on `.git/config` because of the restriction of Git.

```.gitconfig
[include]
	path = <$XDG_CONFIG>/git-profile/<PROFILE-NAME>.gitconfig
```


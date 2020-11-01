# Documentation

## Install

These are the steps to install this git commit-msg hook.

1. Install `rustup` with one of these options:

    - with your operating system's package manager
    - [instructions to install rust and cargo][rustup-install]

[rustup-install]: <https://doc.rust-lang.org/cargo/getting-started/installation.html>

1. Run `rustup update`.

1. Clone this git repository with:
   `git clone https://github.com/sean-hut/amaranth-commit-msg-hook`

1. In the cloned git repository run:

    `cargo build --release`.

1. Copy the binary into the git hook directory of the repository.

    Copy
    `amaranth-commit-msg-hook/target/release/amaranth_commit_msg_hook`
    into the `.git/hooks/`.

1. In the `.git/hooks/` directory rename the binary from `amaranth_commit_msg_hook` to `commit-msg`.

1. Make `.git/hooks/commit-msg` executable with this command:

    `chmod u=rwx .git/hooks/pre-commit`

## Post Install

### Runtime Dependencies

This project has no runtime dependencies.

### Use

This Git hook checks commit messages for their conformance with the Amaranth commit message format.  It will output passed and fail checks.  If you have failed checks refer to the [Amaranth commit message format][amaranth].

[amaranth]: <https://github.com/sean-hut/amaranth-commit-message-format/blob/develop/amaranth-commit-message-format.md>

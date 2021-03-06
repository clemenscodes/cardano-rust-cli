# Cardano Rust CLI

[![License](https://img.shields.io/github/license/clemenshorn/cardano-rust-cli?style=flat)](https://github.com/clemenshorn/cardano-rust-cli/blob/master/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/clemenshorn/cardano-rust-cli/cardano-rust-cli?label=cardano-rust-cli&logo=github&style=flat)](https://github.com/clemenshorn/cardano-rust-cli/actions?query=workflow:cardano-rust-cli)
[![Last Commit](https://img.shields.io/github/last-commit/clemenshorn/cardano-rust-cli?style=flat)](https://github.com/clemenshorn/cardano-rust-cli/graphs/commit-activity)
[![Commits/month](https://img.shields.io/github/commit-activity/m/clemenshorn/cardano-rust-cli?style=flat)](https://github.com/clemenshorn/cardano-rust-cli/graphs/commit-activity)
[![Top Language](https://img.shields.io/github/languages/top/clemenshorn/cardano-rust-cli?style=flat)](https://github.com/clemenshorn/cardano-rust-cli)
[![Repo Size](https://img.shields.io/github/repo-size/clemenshorn/cardano-rust-cli?style=flat)](https://github.com/clemenshorn/cardano-rust-cli)

## Install from source

    git clone https://github.com/clemenshorn/cardano-rust-cli.git
    cd cardano-rust-cli
    cargo build --releas
    sudo mv target/release/cardano /usr/local/bin

## Usage

    cardano <SUBCOMMAND>

## Flags

    -h, --help       Prints help information
    -V, --version    Prints version information

## Subcommands

    help       Prints this message or the help of the given subcommand(s)
    install    Install cardano components
    node       Manage cardano nodes

## Documentation

    cargo doc --open

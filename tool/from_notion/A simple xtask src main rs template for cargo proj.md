---
title: A simple xtask src main rs template for cargo projects
updated: 2026-01-15 07:48:39Z
created: 2026-01-15 04:51:09Z
tags:
  - note
  - rust
---

# A simple xtask/src/main.rs template for cargo projects

Published On: August 10, 2024
Author: dilawar

I often use this template for `xtask/src/main.rs` in all of my rust projects with workspace. File `xtask/Cargo.toml` fis not shown here. 

```rust
use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Debug verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Release mode
    #[arg(short, long, default_value_t = false)]
    release: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build
    Build,
}

fn main() {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    let log_env = match cli.verbosity {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    std::env::set_var("RUST_LOG", log_env);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Build) => {
            build(cli.release).expect("failed to build");
        }
        None => {}
    }
}

fn build(release: bool) -> anyhow::Result<()> {
    build_workspace(release)?;
    build_external(release)?;
    Ok(())
}

fn build_external(release: bool) -> anyhow::Result<()> {
    tracing::info!("Building CXX extension using cmake (release={release})");
    // TODO:
    Ok(())
}

fn build_workspace(release: bool) -> anyhow::Result<()> {
    tracing::info!("Building rust workspace (release={release})");
    let args = if release {
        Some(vec!["--release"])
    } else {
        None
    };
    cargo_cmd("build", args)?;
    Ok(())
}

fn cargo_cmd(subcommand: &str, args: Option<Vec<&str>>) -> anyhow::Result<()> {
    tracing::debug!("Running cargo subcommand {subcommand} with args {args:?}");
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut cmd = std::process::Command::new(cargo);
    cmd.arg(subcommand)
        .arg("--workspace")
        .arg("--exclude")
        .arg("xtask");
    if let Some(args) = args {
        cmd.args(&args);
    }
    let mut child = cmd.spawn()?;
    child.wait()?;
    Ok(())
}
```
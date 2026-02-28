// cli.rs - This file is part of FemtoClaw
// Copyright (c) 2026 FemtoClaw Developers and Contributors
// Description:
//     Talon CLI - Command-line interface for managing FemtoClaw Talons.
//     Provides commands for listing, searching, adding, removing, and
//     discovering talons in the local registry.

//! Talon CLI commands.

use clap::{Parser, Subcommand};
use anyhow::Result;

use crate::TalonRegistry;

#[derive(Parser, Debug)]
#[command(name = "talon")]
#[command(about = "FemtoClaw Talon Manager", long_about = None)]
pub struct Cli {
    #[arg(long, default_value = "./talons")]
    pub dir: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    List,
    Search { query: String },
    Info { name: String },
    Add { path: String },
    Remove { name: String },
    Discover,
    Init,
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    let mut registry = TalonRegistry::from_dir(std::path::PathBuf::from(&cli.dir))?;

    match cli.command {
        Command::List => {
            let talons = registry.list_talons();
            if talons.is_empty() {
                println!("No talons installed. Run `talon discover` to find talons.");
            } else {
                println!("Installed Talons:\n");
                for t in talons {
                    println!("  {} v{} - {}", t.name, t.version, t.description);
                    if let Some(author) = &t.author {
                        println!("    Author: {}", author);
                    }
                    if !t.tags.is_empty() {
                        println!("    Tags: {}", t.tags.join(", "));
                    }
                    println!();
                }
            }
        }
        
        Command::Search { query } => {
            let results = registry.search_talons(&query);
            if results.is_empty() {
                println!("No talons found matching '{}'", query);
            } else {
                println!("Search results for '{}':\n", query);
                for t in results {
                    println!("  {} v{} - {}", t.name, t.version, t.description);
                }
            }
        }
        
        Command::Info { name } => {
            if let Some(t) = registry.get_talon(&name) {
                println!("{}", t.name);
                println!("Version: {}", t.version);
                println!("Description: {}", t.description);
                if let Some(author) = &t.author {
                    println!("Author: {}", author);
                }
                if let Some(license) = &t.license {
                    println!("License: {}", license);
                }
                println!("Path: {}", t.path.display());
            } else {
                println!("Talon '{}' not found", name);
            }
        }
        
        Command::Add { path } => {
            let name = registry.add_talon(std::path::PathBuf::from(path))?;
            println!("Added talon: {}", name);
        }
        
        Command::Remove { name } => {
            registry.remove_talon(&name)?;
            println!("Removed talon: {}", name);
        }
        
        Command::Discover => {
            let discovered = registry.discover_talons()?;
            println!("Discovered {} talon(s)", discovered.len());
            for t in discovered {
                println!("  - {} v{}", t.manifest.name, t.manifest.version);
            }
        }
        
        Command::Init => {
            let dir = std::path::PathBuf::from(&cli.dir);
            if !dir.exists() {
                std::fs::create_dir_all(&dir)?;
            }
            
            let example = dir.join("example-talon").join("TALON.md");
            if !example.exists() {
                let content = r#"---
name: example
version: 1.0.0
description: An example talon demonstrating the format
author: Your Name
license: MIT
tags: [example, demo]
---

# Example Talon

This is an example talon that demonstrates the TALON.md format.

## Commands

### greet
Greets the user with a custom message.

## Requirements
- None

## Usage
This talon can be used to greet users.
"#;
                std::fs::create_dir_all(example.parent().unwrap())?;
                std::fs::write(&example, content)?;
                println!("Created example talon at {}", example.display());
            }
        }
    }

    Ok(())
}


//! Skill CLI commands.

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::SkillRegistry;

#[derive(Parser, Debug)]
#[command(name = "skill")]
#[command(about = "FemtoClaw Skill Manager", long_about = None)]
pub struct Cli {
    #[arg(long, default_value = "./skills")]
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
    let mut registry = SkillRegistry::from_dir(std::path::PathBuf::from(&cli.dir))?;

    match cli.command {
        Command::List => {
            let skills = registry.list_skills();
            if skills.is_empty() {
                println!("No skills installed. Run `skill discover` to find skills.");
            } else {
                println!("Installed Skills:\n");
                for s in skills {
                    println!("  {} v{} - {}", s.name, s.version, s.description);
                    if let Some(author) = &s.author {
                        println!("    Author: {}", author);
                    }
                    if !s.tags.is_empty() {
                        println!("    Tags: {}", s.tags.join(", "));
                    }
                    println!();
                }
            }
        }

        Command::Search { query } => {
            let results = registry.search_skills(&query);
            if results.is_empty() {
                println!("No skills found matching '{}'", query);
            } else {
                println!("Search results for '{}':\n", query);
                for s in results {
                    println!("  {} v{} - {}", s.name, s.version, s.description);
                }
            }
        }

        Command::Info { name } => {
            if let Some(s) = registry.get_skill(&name) {
                println!("{}", s.name);
                println!("Version: {}", s.version);
                println!("Description: {}", s.description);
                if let Some(author) = &s.author {
                    println!("Author: {}", author);
                }
                if let Some(license) = &s.license {
                    println!("License: {}", license);
                }
                println!("Path: {}", s.path.display());
            } else {
                println!("Skill '{}' not found", name);
            }
        }

        Command::Add { path } => {
            let name = registry.add_skill(std::path::PathBuf::from(path))?;
            println!("Added skill: {}", name);
        }

        Command::Remove { name } => {
            registry.remove_skill(&name)?;
            println!("Removed skill: {}", name);
        }

        Command::Discover => {
            let discovered = registry.discover_skills()?;
            println!("Discovered {} skill(s)", discovered.len());
            for s in discovered {
                println!("  - {} v{}", s.manifest.name, s.manifest.version);
            }
        }

        Command::Init => {
            let dir = std::path::PathBuf::from(&cli.dir);
            if !dir.exists() {
                std::fs::create_dir_all(&dir)?;
            }

            let example = dir.join("example-skill").join("SKILL.md");
            if !example.exists() {
                let content = r#"---
name: example
version: 1.0.0
description: An example skill demonstrating the format
author: Your Name
license: MIT
tags: [example, demo]
---

# Example Skill

This is an example skill that demonstrates the SKILL.md format.

## Commands

### greet
Greets the user with a custom message.

## Requirements
- None

## Usage
This skill can be used to greet users.
"#;
                std::fs::create_dir_all(example.parent().unwrap())?;
                std::fs::write(&example, content)?;
                println!("Created example skill at {}", example.display());
            }
        }
    }

    Ok(())
}

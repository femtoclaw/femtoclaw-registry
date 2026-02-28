// lib.rs - This file is part of FemtoClaw
// Copyright (c) 2026 FemtoClaw Developers and Contributors
// Description:
//     FemtoClaw Talon Registry - Core types and traits for Talon management.
//     A Talon is a self-contained capability extension for FemtoClaw.
//     This module defines the manifest format and core data structures.

//! FemtoClaw Talon Registry.
//!
//! A Talon is a self-contained capability extension for FemtoClaw.
//! Talons can be published, versioned, and shared.
//!
//! ## Talon Structure
//!
//! A Talon is a directory containing:
//! - `TALON.md` - Manifest defining the talon
//! - Supporting files (scripts, configs, etc.)
//!
//! ## TALON.md Format
//!
//! ```markdown
//! ---
//! name: github
//! version: 1.0.0
//! description: GitHub integration for issues, PRs, and workflows
//! author: femtoclaw
//! license: MIT
//! tags: [github, devtools, automation]
//! ---
//!
//! # GitHub Talon
//!
//! Provides GitHub integration including:
//! - Issue management
//! - Pull request operations
//! - Workflow triggers
//!
//! ## Requirements
//! - GitHub CLI (gh) installed
//! - GitHub token (GH_TOKEN env var)
//!
//! ## Usage
//! This talon enables FemtoClaw to interact with GitHub repositories.

use serde::{Deserialize, Serialize};

pub mod cli;
pub mod loader;
pub mod registry;

pub use loader::TalonLoader;
pub use registry::{TalonIndex, TalonRegistry};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalonManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub license: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub runtime: Option<TalonRuntime>,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub environment: Vec<EnvVar>,
    #[serde(default)]
    pub commands: Vec<TalonCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalonRuntime {
    pub kind: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub required: bool,
    pub description: Option<String>,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalonCommand {
    pub name: String,
    pub description: String,
    pub args: Vec<CommandArg>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandArg {
    pub name: String,
    pub r#type: String,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalonInfo {
    pub manifest: TalonManifest,
    pub path: std::path::PathBuf,
    pub installed: bool,
}

impl TalonManifest {
    pub fn parse(content: &str) -> anyhow::Result<Self> {
        let mut sections = content.splitn(3, "---");
        let _ = sections.next();
        let frontmatter = sections
            .next()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| anyhow::anyhow!("Invalid TALON.md format: missing frontmatter"))?;

        let manifest: TalonManifest = serde_yaml::from_str(frontmatter)
            .map_err(|e| anyhow::anyhow!("Failed to parse manifest: {}", e))?;

        Ok(manifest)
    }
}

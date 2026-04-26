//! FemtoClaw Skill Registry.
//!
//! A Skill is a self-contained capability extension for FemtoClaw.
//! Skills can be published, versioned, and shared.
//!
//! ## Skill Structure
//!
//! A Skill is a directory containing:
//! - `SKILL.md` - Manifest defining the skill
//! - Supporting files (scripts, configs, etc.)
//!
//! ## SKILL.md Format
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
//! # GitHub Skill
//!
//! Provides GitHub integration including:
//! - Issue management
//! - Pull request operations
//! - Workflow triggers
//! ```

use serde::{Deserialize, Serialize};

pub mod cli;
pub mod loader;
pub mod registry;

pub use loader::SkillLoader;
pub use registry::{SkillEntry, SkillIndex, SkillRegistry};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub license: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub runtime: Option<SkillRuntime>,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub environment: Vec<EnvVar>,
    #[serde(default)]
    pub commands: Vec<SkillCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRuntime {
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
pub struct SkillCommand {
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
pub struct SkillInfo {
    pub manifest: SkillManifest,
    pub path: std::path::PathBuf,
    pub installed: bool,
}

impl SkillManifest {
    pub fn parse(content: &str) -> anyhow::Result<Self> {
        let mut sections = content.splitn(3, "---");
        let _ = sections.next();
        let frontmatter = sections
            .next()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| anyhow::anyhow!("Invalid SKILL.md format: missing frontmatter"))?;

        let manifest: SkillManifest = serde_yaml::from_str(frontmatter)
            .map_err(|e| anyhow::anyhow!("Failed to parse manifest: {}", e))?;

        Ok(manifest)
    }
}

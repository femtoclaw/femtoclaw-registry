// loader.rs - This file is part of FemtoClaw
// Copyright (c) 2026 FemtoClaw Developers and Contributors
// Description:
//     Talon Loader - Load talons into FemtoClaw runtime.
//     Provides functionality for loading talons, retrieving capabilities,
//     and generating system prompts from talon definitions.

//! Talon Loader - Load talons into FemtoClaw.

use anyhow::Result;
use std::path::PathBuf;

use crate::{TalonInfo, TalonManifest, TalonRegistry};

pub struct TalonLoader {
    registry: TalonRegistry,
}

impl TalonLoader {
    pub fn new() -> Result<Self> {
        let registry = TalonRegistry::new()?;
        Ok(Self { registry })
    }

    pub fn from_dir(dir: PathBuf) -> Result<Self> {
        let registry = TalonRegistry::from_dir(dir)?;
        Ok(Self { registry })
    }

    pub fn discover_and_load(&mut self) -> Result<Vec<TalonInfo>> {
        self.registry.discover_talons()
    }

    pub fn load_talon(&self, name: &str) -> Result<TalonInfo> {
        let entry = self
            .registry
            .get_talon(name)
            .ok_or_else(|| anyhow::anyhow!("Talon '{}' not found", name))?;

        let manifest_path = entry.path.join("TALON.md");
        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest = TalonManifest::parse(&content)?;

        Ok(TalonInfo {
            manifest,
            path: entry.path.clone(),
            installed: true,
        })
    }

    pub fn get_capabilities(&self, name: &str) -> Result<Vec<TalonCapability>> {
        let talon = self.load_talon(name)?;
        let mut capabilities = Vec::new();

        for cmd in &talon.manifest.commands {
            capabilities.push(TalonCapability {
                name: format!("{}.{}", name, cmd.name),
                description: cmd.description.clone(),
                args: cmd
                    .args
                    .iter()
                    .map(|a| CapabilityArg {
                        name: a.name.clone(),
                        r#type: a.r#type.clone(),
                        required: a.required,
                    })
                    .collect(),
            });
        }

        Ok(capabilities)
    }

    pub fn generate_system_prompt(&self, names: &[String]) -> Result<String> {
        let mut prompt = String::from("Available Talons:\n\n");

        for name in names {
            if let Ok(talon) = self.load_talon(name) {
                prompt.push_str(&format!(
                    "## {} (v{})\n",
                    talon.manifest.name, talon.manifest.version
                ));
                prompt.push_str(&format!("{}\n\n", talon.manifest.description));

                if !talon.manifest.commands.is_empty() {
                    prompt.push_str("Commands:\n");
                    for cmd in &talon.manifest.commands {
                        prompt.push_str(&format!("- {}: {}\n", cmd.name, cmd.description));
                    }
                    prompt.push('\n');
                }
            }
        }

        Ok(prompt)
    }
}

impl Default for TalonLoader {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            registry: TalonRegistry::default(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct TalonCapability {
    pub name: String,
    pub description: String,
    pub args: Vec<CapabilityArg>,
}

#[derive(Debug, Clone)]
pub struct CapabilityArg {
    pub name: String,
    pub r#type: String,
    pub required: bool,
}

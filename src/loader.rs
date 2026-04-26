//! Skill Loader - Load skills into FemtoClaw.

use anyhow::Result;
use std::path::PathBuf;

use crate::{SkillInfo, SkillManifest, SkillRegistry};

const SKILL_MANIFEST: &str = "SKILL.md";

pub struct SkillLoader {
    registry: SkillRegistry,
}

impl SkillLoader {
    pub fn new() -> Result<Self> {
        let registry = SkillRegistry::new()?;
        Ok(Self { registry })
    }

    pub fn from_dir(dir: PathBuf) -> Result<Self> {
        let registry = SkillRegistry::from_dir(dir)?;
        Ok(Self { registry })
    }

    pub fn discover_and_load(&mut self) -> Result<Vec<SkillInfo>> {
        self.registry.discover_skills()
    }

    pub fn load_skill(&self, name: &str) -> Result<SkillInfo> {
        let entry = self
            .registry
            .get_skill(name)
            .ok_or_else(|| anyhow::anyhow!("Skill '{}' not found", name))?;

        let manifest_path = entry.path.join(SKILL_MANIFEST);
        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest = SkillManifest::parse(&content)?;

        Ok(SkillInfo {
            manifest,
            path: entry.path.clone(),
            installed: true,
        })
    }

    pub fn get_capabilities(&self, name: &str) -> Result<Vec<SkillCapability>> {
        let skill = self.load_skill(name)?;
        let mut capabilities = Vec::new();

        for cmd in &skill.manifest.commands {
            capabilities.push(SkillCapability {
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
        let mut prompt = String::from("Available Skills:\n\n");

        for name in names {
            if let Ok(skill) = self.load_skill(name) {
                prompt.push_str(&format!(
                    "## {} (v{})\n",
                    skill.manifest.name, skill.manifest.version
                ));
                prompt.push_str(&format!("{}\n\n", skill.manifest.description));

                if !skill.manifest.commands.is_empty() {
                    prompt.push_str("Commands:\n");
                    for cmd in &skill.manifest.commands {
                        prompt.push_str(&format!("- {}: {}\n", cmd.name, cmd.description));
                    }
                    prompt.push('\n');
                }
            }
        }

        Ok(prompt)
    }
}

impl Default for SkillLoader {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            registry: SkillRegistry::default(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SkillCapability {
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

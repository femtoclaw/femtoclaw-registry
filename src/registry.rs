//! Skill Registry - Local skill management.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::{SkillInfo, SkillManifest};

const SKILL_MANIFEST: &str = "SKILL.md";
const TALON_MANIFEST: &str = "SKILL.md";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillIndex {
    pub skills: HashMap<String, SkillEntry>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillEntry {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    pub path: PathBuf,
    pub tags: Vec<String>,
}

pub struct SkillRegistry {
    pub skills_dir: PathBuf,
    index: SkillIndex,
}

impl SkillRegistry {
    pub fn new() -> Result<Self> {
        let skills_dir = default_skills_dir()?;

        if !skills_dir.exists() {
            fs::create_dir_all(&skills_dir)?;
        }

        let index = Self::load_index(&skills_dir)?;

        Ok(Self { skills_dir, index })
    }

    pub fn from_dir(dir: PathBuf) -> Result<Self> {
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }

        let index = Self::load_index(&dir)?;

        Ok(Self {
            skills_dir: dir,
            index,
        })
    }

    fn load_index(skills_dir: &Path) -> Result<SkillIndex> {
        let index_path = skills_dir.join("index.json");

        if index_path.exists() {
            let content = fs::read_to_string(&index_path)?;
            let index: SkillIndex = serde_json::from_str(&content)?;
            Ok(index)
        } else {
            Ok(SkillIndex {
                skills: HashMap::new(),
                version: "1.0".to_string(),
            })
        }
    }

    pub fn save_index(&self) -> Result<()> {
        let index_path = self.skills_dir.join("index.json");
        let content = serde_json::to_string_pretty(&self.index)?;
        fs::write(index_path, content)?;
        Ok(())
    }

    pub fn discover_skills(&mut self) -> Result<Vec<SkillInfo>> {
        let mut discovered = Vec::new();

        for entry in WalkDir::new(&self.skills_dir)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file()
                && path
                    .file_name()
                    .is_some_and(|n| n == SKILL_MANIFEST || n == TALON_MANIFEST)
            {
                if let Some(skill_path) = path.parent() {
                    let manifest_path = skill_path.join(path.file_name().unwrap());
                    if let Ok(manifest) = fs::read_to_string(&manifest_path) {
                        if let Ok(manifest) = SkillManifest::parse(&manifest) {
                            let name = manifest.name.clone();

                            let entry = SkillEntry {
                                name: name.clone(),
                                version: manifest.version.clone(),
                                description: manifest.description.clone(),
                                author: manifest.author.clone(),
                                license: manifest.license.clone(),
                                path: skill_path.to_path_buf(),
                                tags: manifest.tags.clone(),
                            };

                            self.index.skills.insert(name.clone(), entry);

                            discovered.push(SkillInfo {
                                manifest,
                                path: skill_path.to_path_buf(),
                                installed: true,
                            });
                        }
                    }
                }
            }
        }

        self.save_index()?;
        Ok(discovered)
    }

    pub fn list_skills(&self) -> Vec<&SkillEntry> {
        self.index.skills.values().collect()
    }

    pub fn get_skill(&self, name: &str) -> Option<&SkillEntry> {
        self.index.skills.get(name)
    }

    pub fn search_skills(&self, query: &str) -> Vec<&SkillEntry> {
        let query_lower = query.to_lowercase();
        self.index
            .skills
            .values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query_lower)
                    || s.description.to_lowercase().contains(&query_lower)
                    || s.tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn add_skill(&mut self, path: PathBuf) -> Result<String> {
        let manifest_path = resolve_manifest_path(&path)?;
        let content = fs::read_to_string(&manifest_path)?;
        let manifest = SkillManifest::parse(&content)?;
        let name = manifest.name.clone();

        let dest = self.skills_dir.join(&name);
        if dest.exists() {
            fs::remove_dir_all(&dest)?;
        }
        fs::create_dir_all(&dest)?;
        copy_dir_recursive(&path, &dest)?;

        if !dest.join(SKILL_MANIFEST).exists() && dest.join(TALON_MANIFEST).exists() {
            fs::rename(dest.join(TALON_MANIFEST), dest.join(SKILL_MANIFEST))?;
        }

        self.index.skills.insert(
            name.clone(),
            SkillEntry {
                name: manifest.name,
                version: manifest.version,
                description: manifest.description,
                author: manifest.author,
                license: manifest.license,
                path: dest,
                tags: manifest.tags,
            },
        );

        self.save_index()?;
        Ok(name)
    }

    pub fn remove_skill(&mut self, name: &str) -> Result<()> {
        if let Some(entry) = self.index.skills.remove(name) {
            if entry.path.exists() {
                fs::remove_dir_all(entry.path)?;
            }
            self.save_index()?;
        }
        Ok(())
    }

    pub fn discover_talons(&mut self) -> Result<Vec<SkillInfo>> {
        self.discover_skills()
    }

    pub fn list_talons(&self) -> Vec<&SkillEntry> {
        self.list_skills()
    }

    pub fn get_talon(&self, name: &str) -> Option<&SkillEntry> {
        self.get_skill(name)
    }

    pub fn search_talons(&self, query: &str) -> Vec<&SkillEntry> {
        self.search_skills(query)
    }

    pub fn add_talon(&mut self, path: PathBuf) -> Result<String> {
        self.add_skill(path)
    }

    pub fn remove_talon(&mut self, name: &str) -> Result<()> {
        self.remove_skill(name)
    }
}

impl Default for SkillRegistry {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            skills_dir: PathBuf::from("./skills"),
            index: SkillIndex::default(),
        })
    }
}

fn default_skills_dir() -> Result<PathBuf> {
    if let Ok(dir) = std::env::var("FEMTO_SKILLS_DIR") {
        return Ok(PathBuf::from(dir));
    }

    if let Ok(dir) = std::env::var("FEMTO_TALONS_DIR") {
        return Ok(PathBuf::from(dir));
    }

    let data_dir = dirs::data_dir().context("Could not find data directory")?;
    Ok(data_dir.join("femtoclaw").join("skills"))
}

fn resolve_manifest_path(path: &Path) -> Result<PathBuf> {
    let skill_md = path.join(SKILL_MANIFEST);
    if skill_md.exists() {
        return Ok(skill_md);
    }

    let talon_md = path.join(TALON_MANIFEST);
    if talon_md.exists() {
        return Ok(talon_md);
    }

    anyhow::bail!(
        "SKILL.md not found in {} (legacy SKILL.md is also accepted)",
        path.display()
    );
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        let relative = entry_path
            .strip_prefix(source)
            .context("Failed to calculate relative skill path")?;

        if relative.as_os_str().is_empty() {
            continue;
        }

        let target_path = destination.join(relative);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target_path)?;
            continue;
        }

        if entry.file_type().is_file() {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry_path, &target_path).with_context(|| {
                format!(
                    "Failed to copy '{}' to '{}'",
                    entry_path.display(),
                    target_path.display()
                )
            })?;
        }
    }

    Ok(())
}

pub type TalonIndex = SkillIndex;
pub type TalonEntry = SkillEntry;
pub type TalonRegistry = SkillRegistry;

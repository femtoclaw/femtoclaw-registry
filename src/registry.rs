// registry.rs - This file is part of FemtoClaw
// Copyright (c) 2026 FemtoClaw Developers and Contributors
// Description:
//     Talon Registry - Local talon management for FemtoClaw.
//     Provides functionality for discovering, adding, removing, and searching
//     for Talons in the local registry.

//! Talon Registry - Local talon management.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::{TalonInfo, TalonManifest};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TalonIndex {
    pub talons: HashMap<String, TalonEntry>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalonEntry {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    pub path: PathBuf,
    pub tags: Vec<String>,
}

pub struct TalonRegistry {
    pub talons_dir: PathBuf,
    index: TalonIndex,
}

impl TalonRegistry {
    pub fn new() -> Result<Self> {
        let talons_dir = dirs::data_dir()
            .context("Could not find data directory")?
            .join("femtoclaw")
            .join("talons");

        if !talons_dir.exists() {
            fs::create_dir_all(&talons_dir)?;
        }

        let index = Self::load_index(&talons_dir)?;

        Ok(Self { talons_dir, index })
    }

    pub fn from_dir(dir: PathBuf) -> Result<Self> {
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }

        let index = Self::load_index(&dir)?;

        Ok(Self {
            talons_dir: dir,
            index,
        })
    }

    fn load_index(talons_dir: &Path) -> Result<TalonIndex> {
        let index_path = talons_dir.join("index.json");

        if index_path.exists() {
            let content = fs::read_to_string(&index_path)?;
            let index: TalonIndex = serde_json::from_str(&content)?;
            Ok(index)
        } else {
            Ok(TalonIndex {
                talons: HashMap::new(),
                version: "1.0".to_string(),
            })
        }
    }

    pub fn save_index(&self) -> Result<()> {
        let index_path = self.talons_dir.join("index.json");
        let content = serde_json::to_string_pretty(&self.index)?;
        fs::write(index_path, content)?;
        Ok(())
    }

    pub fn discover_talons(&mut self) -> Result<Vec<TalonInfo>> {
        let mut discovered = Vec::new();

        for entry in WalkDir::new(&self.talons_dir)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.file_name().is_some_and(|n| n == "TALON.md") {
                if let Some(talon_path) = path.parent() {
                    if let Ok(manifest) = fs::read_to_string(talon_path.join("TALON.md")) {
                        if let Ok(manifest) = TalonManifest::parse(&manifest) {
                            let name = manifest.name.clone();

                            let entry = TalonEntry {
                                name: name.clone(),
                                version: manifest.version.clone(),
                                description: manifest.description.clone(),
                                author: manifest.author.clone(),
                                license: manifest.license.clone(),
                                path: talon_path.to_path_buf(),
                                tags: manifest.tags.clone(),
                            };

                            self.index.talons.insert(name.clone(), entry);

                            discovered.push(TalonInfo {
                                manifest,
                                path: talon_path.to_path_buf(),
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

    pub fn list_talons(&self) -> Vec<&TalonEntry> {
        self.index.talons.values().collect()
    }

    pub fn get_talon(&self, name: &str) -> Option<&TalonEntry> {
        self.index.talons.get(name)
    }

    pub fn search_talons(&self, query: &str) -> Vec<&TalonEntry> {
        let query_lower = query.to_lowercase();
        self.index
            .talons
            .values()
            .filter(|t| {
                t.name.to_lowercase().contains(&query_lower)
                    || t.description.to_lowercase().contains(&query_lower)
                    || t.tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn add_talon(&mut self, path: PathBuf) -> Result<String> {
        let talon_md = path.join("TALON.md");
        if !talon_md.exists() {
            anyhow::bail!("TALON.md not found in {}", path.display());
        }

        let content = fs::read_to_string(&talon_md)?;
        let manifest = TalonManifest::parse(&content)?;
        let name = manifest.name.clone();

        let dest = self.talons_dir.join(&name);
        if dest.exists() {
            fs::remove_dir_all(&dest)?;
        }
        fs::create_dir_all(&dest)?;
        copy_dir_recursive(&path, &dest)?;

        self.index.talons.insert(
            name.clone(),
            TalonEntry {
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

    pub fn remove_talon(&mut self, name: &str) -> Result<()> {
        if let Some(entry) = self.index.talons.remove(name) {
            if entry.path.exists() {
                fs::remove_dir_all(entry.path)?;
            }
            self.save_index()?;
        }
        Ok(())
    }
}

impl Default for TalonRegistry {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            talons_dir: PathBuf::from("./talons"),
            index: TalonIndex::default(),
        })
    }
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        let relative = entry_path
            .strip_prefix(source)
            .context("Failed to calculate relative talon path")?;

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

# FemtoClaw Skill Registry

**Version:** 1.0.3  
**License:** Apache-2.0

---

## Overview

FemtoClaw Skill Registry is the package management system for FemtoClaw Skills. A **Skill** is a self-contained capability extension for FemtoClaw.

This registry system enables:
- **Publishing** Skills to local or remote registries
- **Discovering** available Skills
- **Installing** Skills into your FemtoClaw runtime
- **Versioning** Skills with semantic versioning
- **Searching** Skills by name, tag, or description

---

## Quick Start

```bash
# Install the Skill CLI
cargo install femtoclaw-registry

# Initialize a skills directory
skill init

# Add a Skill from a local directory
skill add ./my-skill

# List installed Skills
skill list

# Search for Skills
skill search github

# Get info about a Skill
skill info github

# Discover Skills in the directory
skill discover

# Remove a Skill
skill remove github
```

---

## What is a Skill?

A Skill is a self-contained extension that provides additional capabilities to FemtoClaw. Think of Skills like apps for your phone - they extend what FemtoClaw can do.

### Skill Structure

```
my-skill/
├── SKILL.md          # Manifest file (required)
├── src/              # Source code (optional)
├── scripts/          # Helper scripts (optional)
└── config/          # Configuration files (optional)
```

### SKILL.md Format

```yaml
---
name: github
version: 1.0.0
description: GitHub integration for issues, PRs, and workflows
author: femtoclaw
license: MIT
tags: [github, devtools, automation]
repository: https://github.com/femtoclaw/skill-github
---

# GitHub Skill

Provides GitHub integration including:
- Issue management
- Pull request operations
- Workflow triggers

## Requirements
- GitHub CLI (gh) installed
- GitHub token (GH_TOKEN env var)

## Usage
This skill enables FemtoClaw to interact with GitHub repositories.
```

### Manifest Fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Unique Skill name (lowercase, hyphenated) |
| `version` | Yes | Semantic version (e.g., 1.0.0) |
| `description` | Yes | One-line description |
| `author` | No | Author name |
| `license` | No | SPDX license identifier |
| `tags` | No | Array of tags for searching |
| `repository` | No | URL to source repository |
| `homepage` | No | URL to homepage |
| `runtime` | No | Runtime requirements |
| `permissions` | No | Required permissions |
| `environment` | No | Required environment variables |
| `commands` | No | Available commands |

---

## Architecture

### Components

```
femtoclaw-registry/
├── src/
│   ├── lib.rs        # Core types and traits
│   ├── registry.rs   # Local registry management
│   ├── loader.rs     # Skill loading and discovery
│   ├── cli.rs        # CLI command implementation
│   └── main.rs       # CLI entry point
```

### Core Types

- **SkillManifest** - Parsed SKILL.md metadata
- **SkillRegistry** - Local Skill index management
- **SkillLoader** - Load and initialize Skills
- **SkillInfo** - Runtime information about a Skill

---

## Configuration

### Skills Directory

By default, Skills are stored in:
- Linux/macOS: `~/.local/share/femtoclaw/skills/`
- Windows: `%APPDATA%/femtoclaw/skills/`

You can override this with the `--dir` flag:
```bash
skill --dir ./my-skills list
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `FEMTO_SKILLS_DIR` | Override default Skills directory |
| `FEMTO_TALONS_DIR` | Legacy alias for the skills directory |

---

## Creating a Skill

### 1. Create the directory structure

```bash
mkdir -p my-skill
cd my-skill
```

### 2. Create SKILL.md

```yaml
---
name: my-skill
version: 0.1.0
description: My custom skill
author: Your Name
license: MIT
tags: [custom, example]
---

# My Skill

Description of what this skill does.

## Usage
How to use this skill.
```

### 3. Add supporting files

- `src/` - Rust source code
- `scripts/` - Shell scripts
- `config/` - Configuration templates

### 4. Test locally

```bash
skill add ./my-skill
skill list
```

---

## Roadmap

- [ ] **v1.1.0** - Remote registry support
- [ ] **v1.2.0** - Skill hub publishing
- [ ] **v1.3.0** - Version constraints
- [ ] **v1.4.0** - Signed skills
- [ ] **v1.5.0** - Plugin system

---

## License

Apache License 2.0 - see LICENSE file for details.

---

## Contributing

Contributions welcome! Please see CONTRIBUTING.md for guidelines.

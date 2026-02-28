# FemtoClaw Talon Registry

**Version:** 1.0.3  
**License:** Apache-2.0

---

## Overview

FemtoClaw Talon Registry is the package management system for FemtoClaw Talons. A **Talon** (from Latin *talon* meaning "claw") is a self-contained capability extension for FemtoClaw.

This registry system enables:
- **Publishing** Talons to local or remote registries
- **Discovering** available Talons
- **Installing** Talons into your FemtoClaw runtime
- **Versioning** Talons with semantic versioning
- **Searching** Talons by name, tag, or description

---

## Quick Start

```bash
# Install the Talon CLI
cargo install femtoclaw-registry

# Initialize a talons directory
talon init

# Add a Talon from a local directory
talon add ./my-talon

# List installed Talons
talon list

# Search for Talons
talon search github

# Get info about a Talon
talon info github

# Discover Talons in the directory
talon discover

# Remove a Talon
talon remove github
```

---

## What is a Talon?

A Talon is a self-contained extension that provides additional capabilities to FemtoClaw. Think of Talons like apps for your phone - they extend what FemtoClaw can do.

### Talon Structure

```
my-talon/
├── TALON.md          # Manifest file (required)
├── src/              # Source code (optional)
├── scripts/          # Helper scripts (optional)
└── config/          # Configuration files (optional)
```

### TALON.md Format

```yaml
---
name: github
version: 1.0.0
description: GitHub integration for issues, PRs, and workflows
author: femtoclaw
license: MIT
tags: [github, devtools, automation]
repository: https://github.com/femtoclaw/talon-github
---

# GitHub Talon

Provides GitHub integration including:
- Issue management
- Pull request operations
- Workflow triggers

## Requirements
- GitHub CLI (gh) installed
- GitHub token (GH_TOKEN env var)

## Usage
This talon enables FemtoClaw to interact with GitHub repositories.
```

### Manifest Fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Unique Talon name (lowercase, hyphenated) |
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
│   ├── loader.rs     # Talon loading and discovery
│   ├── cli.rs        # CLI command implementation
│   └── main.rs       # CLI entry point
```

### Core Types

- **TalonManifest** - Parsed TALON.md metadata
- **TalonRegistry** - Local Talon index management
- **TalonLoader** - Load and initialize Talons
- **TalonInfo** - Runtime information about a Talon

---

## Configuration

### Talons Directory

By default, Talons are stored in:
- Linux/macOS: `~/.local/share/femtoclaw/talons/`
- Windows: `%APPDATA%/femtoclaw/talons/`

You can override this with the `--dir` flag:
```bash
talon --dir ./my-talons list
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `FEMTO_TALONS_DIR` | Override default Talons directory |

---

## Creating a Talon

### 1. Create the directory structure

```bash
mkdir -p my-talon
cd my-talon
```

### 2. Create TALON.md

```yaml
---
name: my-talon
version: 0.1.0
description: My custom talon
author: Your Name
license: MIT
tags: [custom, example]
---

# My Talon

Description of what this talon does.

## Usage
How to use this talon.
```

### 3. Add supporting files

- `src/` - Rust source code
- `scripts/` - Shell scripts
- `config/` - Configuration templates

### 4. Test locally

```bash
talon add ./my-talon
talon list
```

---

## Roadmap

- [ ] **v1.1.0** - Remote registry support
- [ ] **v1.2.0** - TalonHub publishing
- [ ] **v1.3.0** - Version constraints
- [ ] **v1.4.0** - Signed talons
- [ ] **v1.5.0** - Plugin system

---

## License

Apache License 2.0 - see LICENSE file for details.

---

## Contributing

Contributions welcome! Please see CONTRIBUTING.md for guidelines.

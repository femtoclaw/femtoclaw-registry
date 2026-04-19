// main.rs - This file is part of FemtoClaw
// Copyright (c) 2026 FemtoClaw Developers and Contributors
// Description:
//     Main entry point for the Skill CLI tool.
//     Provides command-line interface for managing FemtoClaw Skills.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    femtoclaw_registry::cli::run().await
}

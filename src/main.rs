// main.rs - This file is part of FemtoClaw
// Copyright (c) 2026 FemtoClaw Developers and Contributors
// Description:
//     Main entry point for the Talon CLI tool.
//     Provides command-line interface for managing FemtoClaw Talons.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    femtoclaw_registry::cli::run().await
}

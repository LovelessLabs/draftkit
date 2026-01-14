//! Serve command - runs the MCP server

use anyhow::Result;
use clap::Args;
use rmcp::ServiceExt;

use crate::server::DraftkitServer;

#[derive(Args)]
pub struct ServeArgs {
    // No arguments needed - all data is compiled in
}

/// Run the MCP server
pub async fn cmd_serve(_args: ServeArgs) -> Result<()> {
    tracing::info!("Starting MCP server on stdio");

    let server = DraftkitServer::new();
    let service = server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;

    Ok(())
}

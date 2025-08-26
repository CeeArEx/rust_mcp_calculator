// src/main.rs

mod handler;
mod tools;

use handler::CalculatorServerHandler;
use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, ServerCapabilities, ServerCapabilitiesTools,
    LATEST_PROTOCOL_VERSION,
};

use rust_mcp_sdk::{
    error::SdkResult,
    mcp_server::{server_runtime, ServerRuntime},
    McpServer, StdioTransport, TransportOptions,
};

#[tokio::main]
async fn main() -> SdkResult<()> {
    // STEP 1: Define server details and capabilities
    let server_details = InitializeResult {
        server_info: Implementation {
            name: "Rust Advanced Calculator".to_string(),
            version: "0.1.0".to_string(),
            title: Some("A Production-Ready MCP Calculator Server".to_string()),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
        meta: None,
        instructions: None,
    };

    // STEP 2: Create a stdio transport
    let transport = StdioTransport::new(TransportOptions::default())?;

    // STEP 3: Instantiate our custom handler
    let handler = CalculatorServerHandler {};

    // STEP 4: Create the MCP server runtime
    let server: ServerRuntime = server_runtime::create_server(server_details, transport, handler);

    println!("Rust Advanced Calculator server starting...");

    // STEP 5: Start the server
    if let Err(start_error) = server.start().await {
        eprintln!(
            "{}",
            start_error
                .rpc_error_message()
                .unwrap_or(&start_error.to_string())
        );
    };
    Ok(())
}
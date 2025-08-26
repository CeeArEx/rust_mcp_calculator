// src/handler.rs

use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    schema_utils::CallToolError, CallToolRequest, CallToolResult, ListToolsRequest,
    ListToolsResult, RpcError,
};
use rust_mcp_sdk::{mcp_server::ServerHandler, McpServer};

// Import our custom error type from the tools module
use crate::tools::{CalculatorError, CalculatorTools};

pub struct CalculatorServerHandler;

#[async_trait]
#[allow(unused)]
impl ServerHandler for CalculatorServerHandler {
    async fn handle_list_tools_request(
        &self,
        request: ListToolsRequest,
        runtime: &dyn McpServer,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            tools: CalculatorTools::tools(),
            meta: None,
            next_cursor: None,
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        runtime: &dyn McpServer,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let tool_params: CalculatorTools = CalculatorTools::try_from(request.params)
            .map_err(|e| CallToolError::new(CalculatorError::new(&e.to_string())))?;

        match tool_params {
            CalculatorTools::AddTool(tool) => tool.call_tool(),
            CalculatorTools::DivideTool(tool) => tool.call_tool(),
        }
    }
}
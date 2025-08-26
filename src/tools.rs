// src/tools.rs

use rust_mcp_sdk::schema::{schema_utils::CallToolError, CallToolResult, TextContent};
use rust_mcp_sdk::{
    macros::{mcp_tool, JsonSchema},
    tool_box,
};
use std::error::Error;
use std::fmt;

// --- DEFINE A CUSTOM ERROR TYPE ---
// This is the idiomatic way to handle custom errors in Rust.
#[derive(Debug)]
pub(crate) struct CalculatorError(String);

impl CalculatorError {
    // A constructor function to make creating the error easier.
    pub(crate) fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

// Implement the Display trait so the error can be easily printed.
impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Implement the standard Error trait. This is what CallToolError::new() requires.
impl Error for CalculatorError {}


//****************//
//    AddTool     //
//****************//
#[mcp_tool(
    name = "add",
    description = "Calculates the sum of two numbers.",
    read_only_hint = true
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct AddTool {
    a: f64,
    b: f64,
}

impl AddTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let sum = self.a + self.b;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            sum.to_string(),
        )]))
    }
}

//******************//
//    DivideTool    //
//******************//
#[mcp_tool(
    name = "divide",
    description = "Divides the first number by the second.",
    read_only_hint = true
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct DivideTool {
    a: f64,
    b: f64,
}
impl DivideTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        if self.b == 0.0 {
            return Err(CallToolError::new(CalculatorError::new(
                "Cannot divide by zero.",
            )));
        }
        let result = self.a / self.b;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            result.to_string(),
        )]))
    }
}

//******************//
//  CalculatorTools //
//******************//
tool_box!(CalculatorTools, [AddTool, DivideTool]);


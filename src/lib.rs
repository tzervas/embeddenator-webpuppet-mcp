//! # embeddenator-webpuppet-mcp
//!
//! MCP (Model Context Protocol) server for webpuppet browser automation.
//!
//! This crate provides a standards-compliant MCP server that exposes webpuppet
//! functionality as tools for AI assistants like GitHub Copilot, Claude Desktop,
//! and other MCP-compatible clients.
//!
//! ## Features
//!
//! - **MCP-compliant**: Implements JSON-RPC 2.0 over stdio (standard MCP transport)
//! - **Tool exposure**: Exposes AI prompting, screenshot, and research capabilities
//! - **Security guardrails**: Inherits webpuppet's permission system
//! - **Response screening**: Filters prompt injections and malicious content
//!
//! ## Available Tools
//!
//! - `webpuppet_prompt`: Send prompts to AI providers (Claude, Grok, Gemini)
//! - `webpuppet_screenshot`: Take screenshots of web pages
//! - `webpuppet_list_providers`: List available AI providers
//! - `webpuppet_detect_browsers`: Detect installed browsers
//!
//! ## Usage with VS Code
//!
//! Add to your `.vscode/mcp.json`:
//!
//! ```json
//! {
//!   "servers": {
//!     "webpuppet": {
//!       "command": "webpuppet-mcp",
//!       "args": ["--stdio"],
//!       "env": {}
//!     }
//!   }
//! }
//! ```
//!
//! ## Security Model
//!
//! All operations are subject to the webpuppet permission system:
//! - Destructive operations (delete account, etc.) are blocked
//! - Only allowed domains can be accessed
//! - Responses are screened for prompt injections
//! - All operations are audit logged

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod error;
pub mod protocol;
pub mod server;
pub mod tools;

pub use error::{Error, Result};
pub use protocol::{JsonRpcRequest, JsonRpcResponse, McpMessage};
pub use server::McpServer;
pub use tools::{Tool, ToolRegistry};

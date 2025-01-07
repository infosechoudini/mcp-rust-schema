//! # MCP Rust Schema
//! 
//! This crate provides Rust types and validation for the Model Context Protocol (MCP).
//! It includes JSON schema validation and type definitions for the protocol.
//! 
//! ## Features
//! 
//! - Complete type definitions for MCP messages and data structures
//! - JSON-RPC 2.0 protocol implementation
//! - Schema validation against the MCP specification
//! - Serialization/deserialization support via serde
//! 
//! ## Example
//! 
//! ```rust
//! use mcp_rust_schema::protocol::{Request, RequestId};
//! use mcp_rust_schema::types::RequestMeta;
//! use serde_json::json;
//! 
//! let request = Request::new(
//!     "test/method",
//!     Some(json!({"key": "value"})),
//!     RequestId::String("test-1".to_string()),
//!     Some(RequestMeta {
//!         progress_token: None,
//!     }),
//! );
//! ```
//! 
//! For more examples, see the `tests` directory.

pub mod error;
pub mod protocol;
pub mod types;

# Model Context Protocol (MCP) Rust Schema

A Rust implementation of the Model Context Protocol (MCP) schema, providing type definitions and validation for MCP messages.

[![crates.io](https://img.shields.io/crates/v/mcp_rust_schema.svg)](https://crates.io/crates/mcp_rust_schema)
[![Documentation](https://docs.rs/mcp_rust_schema/badge.svg)](https://docs.rs/mcp_rust_schema)

## Features

- 🔍 Complete type definitions for MCP messages and data structures
- 🚀 JSON-RPC 2.0 message schema support
- ✅ Schema validation against the MCP specification
- 📦 Serialization/deserialization support via serde

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mcp_rust_schema = "0.1.4"
```

## Usage

### Basic Message Creation

```rust
use mcp_rust_schema::protocol::{Request, RequestId};
use mcp_rust_schema::types::RequestMeta;
use serde_json::json;

// Create a new request
let request = Request::new(
    "test/method",
    Some(json!({"key": "value"})),
    RequestId::String("test-1".to_string()),
    Some(RequestMeta {
        progress_token: None,
    }),
);
```

### Schema Validation

```rust
use mcp_rust_schema::types::Resource;
use serde_json::json;
use jsonschema::Validator;

// Create a resource
let resource = Resource {
    uri: "file:///test.txt".to_string(),
    title: "Test File".to_string(),
    description: Some("A test file".to_string()),
    contents: ResourceContents::Text {
        text: "Hello World".to_string(),
        uri: "file:///test.txt".to_string(),
        mime_type: Some("text/plain".to_string()),
    },
    mime_type: Some("text/plain".to_string()),
    annotations: None,
};

// Validate against schema
let resource_json = serde_json::to_value(resource)?;
let validator = jsonschema::validator_for(&schema)?;
assert!(validator.is_valid(&resource_json));
```

## Error Handling

The library provides comprehensive error handling:

```rust
use mcp_rust_schema::error::Error;

match result {
    Ok(value) => println!("Success: {:?}", value),
    Err(Error::Protocol { code, message, .. }) => println!("Protocol error {}: {}", code, message),
    Err(Error::Serialization(e)) => println!("Serialization error: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

use serde_json::{json, Value};
use std::sync::OnceLock;

use mcp_rust_schema::protocol::*;
use mcp_rust_schema::types::*;

static SCHEMA: OnceLock<Value> = OnceLock::new();

fn get_schema() -> &'static Value {
    SCHEMA.get_or_init(|| {
        let schema_str = include_str!("./mcp_schema.json");
        serde_json::from_str(schema_str)
            .expect("Failed to parse schema JSON")
    })
}

fn validate_json(value: &Value) -> Result<(), String> {
    let schema = get_schema();
    let validator = jsonschema::validator_for(schema)
        .map_err(|e| format!("Failed to create validator: {}", e))?;
    
    if !validator.is_valid(value) {
        let mut error_msg = String::new();
        for error in validator.iter_errors(value) {
            error_msg.push_str(&format!("\nError: {}\nLocation: {}", error, error.instance_path));
        }
        return Err(error_msg);
    }
    Ok(())
}

#[test]
fn test_request_matches_schema() -> Result<(), String> {
    let request = Request::new(
        "test/method",
        Some(json!({"key": "value"})),
        RequestId::String("test-1".to_string()),
        Some(RequestMeta {
            progress_token: Some(ProgressToken::String("token1".to_string())),
        }),
    );
    
    let request_json = serde_json::to_value(request)
        .map_err(|e| format!("Failed to serialize request: {}", e))?;
    validate_json(&request_json)
}

#[test]
fn test_response_matches_schema() -> Result<(), String> {
    let response = Response::success(
        RequestId::Number(1),
        Some(json!({"result": "success"})),
        Some(ResponseMeta {
            custom: Some(std::collections::HashMap::new()),
        }),
    );
    
    let response_json = serde_json::to_value(response)
        .map_err(|e| format!("Failed to serialize response: {}", e))?;
    validate_json(&response_json)
}

#[test]
fn test_notification_matches_schema() -> Result<(), String> {
    let notification = Notification::new(
        "test/event",
        Some(json!({"event": "update"})),
        Some(NotificationMeta {
            custom: Some(std::collections::HashMap::new()),
        }),
    );
    
    let notification_json = serde_json::to_value(notification)
        .map_err(|e| format!("Failed to serialize notification: {}", e))?;
    validate_json(&notification_json)
}

#[test]
fn test_resource_matches_schema() -> Result<(), String> {
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
        annotations: Some(Annotations {
            audience: Some(vec![Role::User]),
            priority: Some(0.5),
        }),
    };
    
    let resource_json = serde_json::to_value(resource)
        .map_err(|e| format!("Failed to serialize resource: {}", e))?;
    validate_json(&resource_json)
}

#[test]
fn test_complete_request_matches_schema() -> Result<(), String> {
    let complete_request = CompleteRequest {
        prompt_id: "test-prompt".to_string(),
        arguments: Some(std::collections::HashMap::new()),
        model_preferences: Some(ModelPreferences {
            cost_priority: Some(0.5),
            speed_priority: Some(0.8),
            intelligence_priority: Some(0.7),
            hints: Some(vec![ModelHint {
                name: Some("gpt-4".to_string()),
            }]),
        }),
    };
    
    let request_json = serde_json::to_value(complete_request)
        .map_err(|e| format!("Failed to serialize complete request: {}", e))?;
    validate_json(&request_json)
} 
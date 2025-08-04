use anyhow::Result;
use serde_json;
use std::{io::Write, path::Path};
use tempfile::NamedTempFile;

use crate::{RequestMessage, RequestPayload, env_api_key, known_endpoints, read_file_content, schema::ApiResponse};

#[test]
fn test_read_file_content() -> Result<()> {
    // Create a temporary file
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "Hello, world!")?;

    // Read the file content
    let content = read_file_content(temp_file.path())?;
    assert_eq!(content.trim(), "Hello, world!");

    // Test file not found
    let result = read_file_content(Path::new("non_existent_file.txt"));
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_read_file_content_edge_cases() -> Result<()> {
    // Test reading an empty file
    let mut temp_file = NamedTempFile::new()?;
    let content = read_file_content(temp_file.path())?;
    assert!(content.is_empty());

    // Test reading a file with multiple lines
    writeln!(temp_file, "Line 1")?;
    writeln!(temp_file, "Line 2")?;
    let content = read_file_content(temp_file.path())?;
    assert_eq!(content.trim(), "Line 1\nLine 2");

    // Test reading a non-existent file
    let result = read_file_content(Path::new("non_existent_file.txt"));
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_read_large_file_content() -> Result<()> {
    // Create a large temporary file
    let mut temp_file = NamedTempFile::new()?;
    for _ in 0..10000 {
        writeln!(temp_file, "Hello, world!")?;
    }

    // Read the file content
    let content = read_file_content(temp_file.path())?;
    assert_eq!(content.lines().count(), 10000);

    Ok(())
}

#[test]
fn test_known_endpoints_with_various_inputs() {
    // Test known endpoints
    assert_eq!(known_endpoints("openai"), "https://api.openai.com/v1/chat/completions");
    assert_eq!(
        known_endpoints("google"),
        "https://generativelanguage.googleapis.com/v1beta/chat/completions"
    );
    assert_eq!(
        known_endpoints("hf"),
        "https://router.huggingface.co/v1/chat/completions"
    );

    // Test unknown endpoint
    assert_eq!(known_endpoints("unknown"), "none");

    // Test empty string
    assert_eq!(known_endpoints(""), "none");

    // Test endpoint with whitespace
    assert_eq!(known_endpoints(" openai "), "none");
}

#[test]
fn test_env_api_key_with_various_inputs() {
    // Test known endpoints
    assert_eq!(env_api_key("openai"), "API_TOKEN_OAI");
    assert_eq!(env_api_key("google"), "API_TOKEN_GOOGLE");
    assert_eq!(env_api_key("hf"), "API_TOKEN_HF");

    // Test unknown endpoint
    assert_eq!(env_api_key("unknown"), "API_TOKEN");

    // Test empty string
    assert_eq!(env_api_key(""), "API_TOKEN");

    // Test endpoint with whitespace
    assert_eq!(env_api_key(" openai "), "API_TOKEN");
}

#[test]
fn test_known_endpoints() {
    assert_eq!(known_endpoints("openai"), "https://api.openai.com/v1/chat/completions");
    assert_eq!(
        known_endpoints("google"),
        "https://generativelanguage.googleapis.com/v1beta/chat/completions"
    );
    assert_eq!(
        known_endpoints("hf"),
        "https://router.huggingface.co/v1/chat/completions"
    );
    assert_eq!(known_endpoints("unknown"), "none");
}

#[test]
fn test_env_api_key() {
    assert_eq!(env_api_key("openai"), "API_TOKEN_OAI");
    assert_eq!(env_api_key("google"), "API_TOKEN_GOOGLE");
    assert_eq!(env_api_key("hf"), "API_TOKEN_HF");
    assert_eq!(env_api_key("unknown"), "API_TOKEN");
}

#[test]
fn test_known_endpoints_with_custom_url() {
    assert_eq!(known_endpoints("https://custom.endpoint.com"), "none");
}

#[test]
fn test_env_api_key_with_custom_endpoint() {
    assert_eq!(env_api_key("https://custom.endpoint.com"), "API_TOKEN");
}

#[test]
fn test_request_message_serialization() -> Result<()> {
    let message = RequestMessage {
        role: "assistant".to_owned(),
        content: "Hello, world!".to_owned(),
    };

    let json = serde_json::to_string(&message)?;
    assert_eq!(json, "{\"role\":\"assistant\",\"content\":\"Hello, world!\"}");

    Ok(())
}

#[test]
fn test_request_payload_serialization() -> Result<()> {
    let mut messages = vec![RequestMessage {
        role: "assistant".to_owned(),
        content: "Hello, world!".to_owned(),
    }];

    messages.push(RequestMessage {
        role: "user".to_owned(),
        content: "Hi!".to_owned(),
    });

    let payload = RequestPayload {
        messages,
        model: "test_model",
        max_tokens: Some(100),
        max_completion_tokens: None,
    };

    let json = serde_json::to_string(&payload)?;
    assert_eq!(
        json,
        "{\"messages\":[{\"role\":\"assistant\",\"content\":\"Hello, \
         world!\"},{\"role\":\"user\",\"content\":\"Hi!\"}],\"model\":\"test_model\",\"max_tokens\":100}"
    );

    Ok(())
}

#[test]
fn test_api_response_deserialization() -> Result<()> {
    let json = "{\"id\":\"test_id\",\"object\":\"chat.completion\",\"created\":1643723400,\"model\":\"test_model\",\"\
                choices\":[{\"index\":0,\"message\":{\"role\":\"assistant\",\"content\":\"Hello, \
                world!\"},\"finish_reason\":\"stop\"}],\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":20,\"\
                total_tokens\":30}}";

    let response: ApiResponse = serde_json::from_str(json)?;
    assert_eq!(response.id, "test_id");
    assert_eq!(response.object, "chat.completion");
    assert_eq!(response.created, 1643723400);
    assert_eq!(response.model, "test_model");
    assert_eq!(response.choices.len(), 1);
    assert_eq!(response.usage.prompt_tokens, 10);
    assert_eq!(response.usage.completion_tokens, 20);
    assert_eq!(response.usage.total_tokens, 30);

    Ok(())
}

#[test]
fn test_request_payload_serialization_with_different_token_settings() -> Result<()> {
    let mut messages = vec![RequestMessage {
        role: "assistant".to_owned(),
        content: "Hello, world!".to_owned(),
    }];

    messages.push(RequestMessage {
        role: "user".to_owned(),
        content: "Hi!".to_owned(),
    });

    // Test with max_tokens
    let payload = RequestPayload {
        messages: messages.clone(),
        model: "test_model",
        max_tokens: Some(100),
        max_completion_tokens: None,
    };

    let json = serde_json::to_string(&payload)?;
    assert_eq!(
        json,
        "{\"messages\":[{\"role\":\"assistant\",\"content\":\"Hello, \
         world!\"},{\"role\":\"user\",\"content\":\"Hi!\"}],\"model\":\"test_model\",\"max_tokens\":100}"
    );

    // Test with max_completion_tokens
    let payload = RequestPayload {
        messages: messages.clone(),
        model: "test_model",
        max_tokens: None,
        max_completion_tokens: Some(100),
    };

    let json = serde_json::to_string(&payload)?;
    assert_eq!(
        json,
        "{\"messages\":[{\"role\":\"assistant\",\"content\":\"Hello, \
         world!\"},{\"role\":\"user\",\"content\":\"Hi!\"}],\"model\":\"test_model\",\"max_completion_tokens\":100}"
    );

    // Test with both max_tokens and max_completion_tokens
    let payload = RequestPayload {
        messages: messages.clone(),
        model: "test_model",
        max_tokens: Some(100),
        max_completion_tokens: Some(100),
    };

    let json = serde_json::to_string(&payload)?;
    assert_eq!(
        json,
        "{\"messages\":[{\"role\":\"assistant\",\"content\":\"Hello, \
         world!\"},{\"role\":\"user\",\"content\":\"Hi!\"}],\"model\":\"test_model\",\"max_tokens\":100,\"\
         max_completion_tokens\":100}"
    );

    Ok(())
}

#[test]
fn test_request_payload_serialization_with_empty_messages() -> Result<()> {
    let payload = RequestPayload {
        messages: vec![],
        model: "test_model",
        max_tokens: Some(100),
        max_completion_tokens: None,
    };

    let json = serde_json::to_string(&payload)?;
    assert_eq!(json, "{\"messages\":[],\"model\":\"test_model\",\"max_tokens\":100}");

    Ok(())
}

#[test]
fn test_request_payload_serialization_with_multiple_messages() -> Result<()> {
    let mut messages = vec![];
    for i in 0..10 {
        messages.push(RequestMessage {
            role: "assistant".to_owned(),
            content: format!("Message {}", i),
        });
    }

    let payload = RequestPayload {
        messages,
        model: "test_model",
        max_tokens: Some(100),
        max_completion_tokens: None,
    };

    let json = serde_json::to_string(&payload)?;
    assert!(json.contains("\"messages\":["));
    assert!(json.contains("\"model\":\"test_model\""));
    assert!(json.contains("\"max_tokens\":100"));

    Ok(())
}

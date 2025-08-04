use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Response structure from the chat completion API.
///
/// This structure represents the complete JSON response from the API,
/// including the generated content, usage statistics, and metadata.
///
/// # Fields
/// * `id` - Unique identifier for the API response
/// * `object` - Type of object returned (typically "chat.completion")
/// * `created` - Unix timestamp of when the response was created
/// * `model` - Identifier of the model that generated the response
/// * `choices` - Vector of completion choices (typically contains one item)
/// * `usage` - Token usage statistics for the request
/// * `service_tier` - Service tier information (if available)
/// * `system_fingerprint` - System fingerprint for the response (if available)
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    #[serde(rename = "service_tier")]
    pub service_tier: Option<String>,
    #[serde(rename = "system_fingerprint")]
    pub system_fingerprint: Option<String>,
}

/// Represents a single completion choice in the API response.
///
/// Each choice contains the generated message, finish reason, and additional
/// metadata about the completion process.
///
/// # Fields
/// * `index` - Index of this choice in the response (typically 0)
/// * `message` - The generated message content
/// * `finish_reason` - Reason why the completion finished (e.g., "stop",
///   "length")
/// * `logprobs` - Log probability information (if requested)
/// * `content_filter_results` - Results from content filtering (if applicable)
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub index: i64,
    pub message: Message,
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    pub logprobs: Option<Value>,
    #[serde(rename = "content_filter_results")]
    pub content_filter_results: Option<ContentFilterResults>,
}

/// Represents a message in the API response.
///
/// Contains the role and content of a message, along with optional refusal
/// and annotation information.
///
/// # Fields
/// * `role` - Role of the message sender ("assistant", "user", etc.)
/// * `content` - The actual content of the message
/// * `refusal` - Reason for refusal if the request was refused
/// * `annotations` - Additional annotations (if any)
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub refusal: Option<Value>,
    pub annotations: Option<Vec<Value>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentFilterResults {
    pub hate: Hate,
    pub self_harm: SelfHarm,
    pub sexual: Sexual,
    pub violence: Violence,
    pub jailbreak: Jailbreak,
    pub profanity: Profanity,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hate {
    pub filtered: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfHarm {
    pub filtered: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sexual {
    pub filtered: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Violence {
    pub filtered: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Jailbreak {
    pub filtered: bool,
    pub detected: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profanity {
    pub filtered: bool,
    pub detected: bool,
}

/// Token usage statistics for the API request.
///
/// Provides detailed information about token consumption for both
/// the prompt and completion portions of the request.
///
/// # Fields
/// * `prompt_tokens` - Number of tokens in the prompt
/// * `completion_tokens` - Number of tokens in the completion
/// * `total_tokens` - Total number of tokens used
/// * `prompt_tokens_details` - Detailed breakdown of prompt tokens
/// * `completion_tokens_details` - Detailed breakdown of completion tokens
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
    pub prompt_tokens_details: Option<PromptTokensDetails>,
    pub completion_tokens_details: Option<CompletionTokensDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptTokensDetails {
    pub cached_tokens: Option<i64>,
    pub audio_tokens: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::struct_field_names)]
pub struct CompletionTokensDetails {
    pub reasoning_tokens: Option<i64>,
    pub audio_tokens: Option<i64>,
    pub accepted_prediction_tokens: Option<i64>,
    pub rejected_prediction_tokens: Option<i64>,
}

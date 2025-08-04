mod schema;
#[cfg(test)]
mod tests;

use anyhow::{Context, Result, bail};
use clap::Parser;
use log::{info, warn};
use reqwest::Client;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::time::Instant;

use crate::schema::ApiResponse;

/// Default endpoint value used when no known endpoint name is provided.
/// This constant serves as a fallback to indicate that a custom endpoint URL
/// should be used.
const DEFAULT_ENDPOINT: &str = "none";

/// Role identifier for assistant messages in the chat completion API.
/// Used to distinguish AI-generated responses from user inputs in the message
/// history.
const ASSISTANT_ROLE: &str = "assistant";

/// Role identifier for user messages in the chat completion API.
/// Used to distinguish user inputs from AI-generated responses in the message
/// history.
const USER_ROLE: &str = "user";

/// Command-line argument parser for the application.
///
/// This structure defines all the required and optional parameters that can be
/// passed to the application via command line. It uses clap's derive macro to
/// automatically generate the argument parsing logic.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Query an OpenAI-compatible endpoint with a prompt and input file, writing the response to an output \
                  file."
)]
struct Args {
    /// The API endpoint name (e.g., "openai", "google") or a custom URL to
    /// query.
    #[arg(short, long, required = true)]
    endpoint: String,

    /// The model identifier to use for the completion.
    #[arg(short, long, required = true)]
    model: String,

    /// Whether to use reasoning tokens instead of regular max tokens.
    #[arg(short, long, required = false)]
    reasoning: bool,

    /// Maximum number of tokens to generate.
    #[arg(short, long, required = true)]
    tokens: u32,

    /// Path to the file containing the system prompt.
    #[arg(short, long, value_parser, required = true)]
    prompt: PathBuf,

    /// Path to the file containing the user input.
    #[arg(short, long, value_parser, required = true)]
    input: PathBuf,

    /// Optional path to save the response (prints to stdout if not provided).
    #[arg(short, value_parser, required = false)]
    output: Option<PathBuf>,
}

/// Represents a single message in the chat completion request.
///
/// Each message consists of a role (either "user" or "assistant") and content.
/// Messages are used to provide context and instructions to the AI model.
#[derive(Serialize, Debug, Clone)]
struct RequestMessage {
    role: String,
    content: String,
}

/// The complete request payload sent to the chat completion API.
///
/// This structure contains all the necessary parameters for making a chat
/// completion request, including the message history, model identifier, and
/// token limits.
///
/// # Fields
/// * `messages` - Vector of messages providing context for the completion
/// * `model` - Identifier of the model to use for generation
/// * `max_tokens` - Maximum number of tokens to generate (used for regular
///   models)
/// * `max_completion_tokens` - Maximum number of completion tokens (used for
///   reasoning models)
#[derive(Serialize, Debug)]
struct RequestPayload<'a> {
    messages: Vec<RequestMessage>,
    model: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_completion_tokens: Option<u32>,
}

/// Reads the entire contents of a file into a string.
///
/// This function attempts to read all text content from the specified file path
/// and returns it as a String. If the file cannot be read, an error is returned
/// with context about what went wrong.
///
/// # Arguments
/// * `file_path` - Path to the file to read (can be any type that implements
///   `AsRef<Path>`)
///
/// # Returns
/// * `Ok(String)` - The file contents as a string
/// * `Err` - An error if the file could not be read
///
/// # Examples
/// ```
/// let content = read_file_content("example.txt")?;
fn read_file_content(file_path: impl AsRef<Path>) -> Result<String> {
    fs::read_to_string(file_path).context("Failed to read file content")
}

/// Maps known endpoint names to their corresponding API URLs.
///
/// This function takes a string identifier for a known service and returns
/// the appropriate API endpoint URL. If the name is not recognized, it returns
/// the `DEFAULT_ENDPOINT` constant.
///
/// # Arguments
/// * `name` - The endpoint name to look up ("openai", "google", "hf", etc.)
///
/// # Returns
/// * The corresponding API URL, or `DEFAULT_ENDPOINT` if name is not recognized
///
/// # Supported Endpoints
/// * "openai" - `OpenAI` API endpoint
/// * "google" - Google Generative Language API endpoint
/// * "hf" - Hugging Face API endpoint
fn known_endpoints(name: &str) -> &str {
    match name {
        "openai" => "https://api.openai.com/v1/chat/completions",
        "google" => "https://generativelanguage.googleapis.com/v1beta/chat/completions",
        "hf" => "https://router.huggingface.co/v1/chat/completions",
        _ => DEFAULT_ENDPOINT,
    }
}

/// Maps endpoint names to their corresponding environment variable names for
/// API keys.
///
/// This function determines which environment variable should be checked for
/// the API key based on the endpoint being used. This allows different services
/// to use different environment variables for their authentication tokens.
///
/// # Arguments
/// * `name` - The endpoint name to look up ("openai", "google", "hf", etc.)
///
/// # Returns
/// * The corresponding environment variable name for the API key
///
/// # Environment Variables
/// * "openai" - Uses `API_TOKEN_OAI`
/// * "google" - Uses `API_TOKEN_GOOGLE`
/// * "hf" - Uses `API_TOKEN_HF`
/// * All others - Uses `API_TOKEN` (default)
fn env_api_key(name: &str) -> &str {
    match name {
        "openai" => "API_TOKEN_OAI",
        "google" => "API_TOKEN_GOOGLE",
        "hf" => "API_TOKEN_HF",
        _ => "API_TOKEN",
    }
}

/// Main application entry point.
///
/// This function orchestrates the entire application workflow:
/// 1. Parses command-line arguments
/// 2. Validates input parameters
/// 3. Reads prompt and input files
/// 4. Constructs the API request payload
/// 5. Sends the request to the specified endpoint
/// 6. Processes and outputs the response
///
/// # Workflow
/// 1. Initialize logging and timing
/// 2. Parse command-line arguments using Args struct
/// 3. Validate token count is greater than 0
/// 4. Retrieve API key from environment variables
/// 5. Read prompt and input file contents
/// 6. Construct message history with assistant prompt and user input
/// 7. Build request payload with appropriate token limits
/// 8. Determine API endpoint URL
/// 9. Send HTTP POST request to API
/// 10. Handle API response and output results
///
/// # Environment Variables
/// * `API_TOKEN_OAI` - `OpenAI` API key
/// * `API_TOKEN_GOOGLE` - `Google` API key
/// * `API_TOKEN_HF` - `Hugging Face` API key
/// * `API_TOKEN` - Default API key for custom endpoints
///
/// # Returns
/// * `Ok(())` on successful completion
/// * `Err` on any failure during execution
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let start_time = Instant::now();
    let args = Args::parse();

    if args.tokens == 0 {
        bail!("Token count must be greater than 0");
    }

    let api_key_name = env_api_key(&args.endpoint);
    let api_token = env::var(api_key_name)
        .with_context(|| format!("{api_key_name} variable not set. Please provide your API token."))?;

    let prompt_content = read_file_content(args.prompt)?;
    if prompt_content.is_empty() {
        bail!("Prompt content from prompt file is empty.");
    }

    let input_content = read_file_content(args.input)?;
    if input_content.is_empty() {
        bail!("Input content from input file is empty.");
    }

    let mut messages = vec![RequestMessage {
        role: ASSISTANT_ROLE.to_owned(),
        content: prompt_content,
    }];

    messages.push(RequestMessage {
        role: USER_ROLE.to_owned(),
        content: input_content,
    });

    let payload = RequestPayload {
        messages,
        model: &args.model,
        max_tokens: if args.reasoning { None } else { Some(args.tokens) },
        max_completion_tokens: if args.reasoning { Some(args.tokens) } else { None },
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .context("Failed to create HTTP client")?;
    let auth_header_value = format!("Bearer {api_token}");

    let mut api_url = known_endpoints(&args.endpoint);
    if api_url == "none" {
        api_url = &args.endpoint;
    }

    info!("Querying model '{}' model", args.model);
    info!("With URL: '{api_url}'");

    let response = client
        .post(api_url)
        .header("Authorization", &auth_header_value)
        .json(&payload)
        .send()
        .await
        .context("Failed to send request to the API.")?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.context("Could not read error body")?;
        bail!("API request failed with status {}: {}", status, error_body);
    }

    let api_response: ApiResponse = response
        .json()
        .await
        .context("Failed to parse JSON response from the API.")?;

    if let Some(first_choice) = api_response.choices.first() {
        let content = &first_choice.message.content;
        match &args.output {
            Some(path) => {
                fs::write(path, content)?;
                info!("API response successfully saved to output file");
            },
            None => println!("{content}"), // Consistent output
        }
    } else {
        warn!("API returned a response, but it contained no choices.");
    }

    info!("Time elapsed: {:.2?}", start_time.elapsed());

    Ok(())
}

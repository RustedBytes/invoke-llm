# Invoke LLM

[![badges](https://img.shields.io/badge/open-all_badges-green)](./BADGES.md)

A command-line tool for querying OpenAI-compatible endpoints with a prompt and
input file, and writing the response to an output file.

## Table of Contents

* [Installation](#installation)
* [Usage](#usage)
* [Command-Line Arguments](#command-line-arguments)
* [Environment Variables](#environment-variables)
* [Supported Endpoints](#supported-endpoints)
* [Examples](#examples)
* [Development](#development)
* [Building and Testing](#building-and-testing)
* [Contributing](#contributing)
* [Troubleshooting](#troubleshooting)

### Installation

To install `invoke-llm`, run the following command:

```bash
cargo install --git https://github.com/RustedBytes/invoke-llm
```

### Usage

`invoke-llm` is a command-line tool that queries an OpenAI-compatible endpoint
with a prompt and input file, and writes the response to an output file. The
basic usage is as follows:

```bash
invoke-llm --endpoint <endpoint> --model <model> --tokens <tokens> --prompt <prompt_file> --input <input_file> [--output <output_file>]
```

### Command-Line Arguments

The following command-line arguments are supported:

* `--endpoint` (required): The API endpoint name (e.g., "openai", "google") or a
  custom URL to query.
* `--model` (required): The model identifier to use for the completion.
* `--tokens` (required): Maximum number of tokens to generate.
* `--prompt` (required): Path to the file containing the system prompt.
* `--input` (required): Path to the file containing the user input.
* `--output` (optional): Path to save the response (prints to stdout if not
  provided).
* `--reasoning` (optional): Whether to use reasoning models instead of regular
  ones.

### Environment Variables

The following environment variables are used to store API keys:

* `API_TOKEN_OAI`: OpenAI API key
* `API_TOKEN_GOOGLE`: Google API key
* `API_TOKEN_HF`: Hugging Face API key
* `API_TOKEN`: Default API key for custom endpoints

### Supported Endpoints

The following endpoints are currently supported:

* "openai": OpenAI API endpoint
* "google": Google Generative Language API endpoint
* "hf": Hugging Face API endpoint
* Custom endpoints: Any custom URL can be used as an endpoint

### Examples

To run `invoke-llm` with some pre-defined prompts, use the following commands:

```bash
just -f llmfile code_review
just -f llmfile gemma_grammar_check
```

## Development

To contribute to this project, you'll need:

1. Rust toolchain (nightly version recommended)
2. `cargo install action-validator dircat just`
3. `cargo install --git https://github.com/ytmimi/markdown-fmt markdown-fmt
   --features="build-binary"`
4. `brew install lefthook` (for pre-commit hooks)
5. [yamlfmt](https://github.com/google/yamlfmt) (for YAML formatting)

## Building and Testing

1. Clone the repository
2. Run `cargo build` to compile the application
3. Run `cargo test` to execute the test suite
4. Run `cargo run -- --help` to see command-line options

To test `invoke-llm` with different endpoints, use the following commands:

```bash
just -f llmfile.test gemini
just -f llmfile.test gemma
just -f llmfile.test hf
just -f llmfile.test oai
just -f llmfile.test oai_reasoning
```

## Contributing

Contributions are welcome! Please submit pull requests with clear descriptions
of changes and ensure that all tests pass before submitting.

## Troubleshooting

* Check you have correct API keys
* For other issues, please check the [issues
  page](https://github.com/RustedBytes/invoke-llm/issues) or submit a new
  issue with detailed information about your problem.

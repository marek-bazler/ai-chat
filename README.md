# AI Chat CLI

A simple command-line AI chat application built in Rust with support for multiple providers and configurable API keys.

## Features

- 🔧 Easy configuration of API keys and models
- 🤖 Support for OpenAI and Anthropic providers
- 💬 Interactive chat sessions
- 🎯 Model selection per session
- 📁 Secure config storage

## Installation

```bash
cargo build --release
```

## Usage

### Configure API Keys

First, set up your API keys and select models:

```bash
cargo run -- config
```

This will prompt you to:
1. Select a provider (OpenAI or Anthropic)
2. Enter your API key
3. Choose a default model

### Start Chatting

Start a chat session:

```bash
# Use default provider and model
cargo run -- chat

# Specify provider
cargo run -- chat --provider openai

# Specify both provider and model
cargo run -- chat --provider openai --model gpt-4
```

### Supported Providers

**OpenAI:**
- gpt-4
- gpt-4-turbo
- gpt-3.5-turbo

**Anthropic:**
- claude-3-opus-20240229
- claude-3-sonnet-20240229
- claude-3-haiku-20240307

## Configuration

Configuration is stored in your system's config directory:
- Linux: `~/.config/ai-chat/config.json`
- macOS: `~/Library/Application Support/ai-chat/config.json`
- Windows: `%APPDATA%\ai-chat\config.json`

## Getting API Keys

- **OpenAI**: Get your API key from [OpenAI Platform](https://platform.openai.com/api-keys)
- **Anthropic**: Get your API key from [Anthropic Console](https://console.anthropic.com/)
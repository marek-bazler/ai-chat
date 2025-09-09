use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct ChatClient {
    client: Client,
    provider: String,
    api_key: String,
    model: String,
}

#[derive(Serialize, Deserialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
}

#[derive(Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Deserialize)]
struct AnthropicContent {
    text: String,
}

impl ChatClient {
    pub fn new(provider: &str, api_key: &str, model: &str) -> Self {
        Self {
            client: Client::new(),
            provider: provider.to_string(),
            api_key: api_key.to_string(),
            model: model.to_string(),
        }
    }
    
    pub async fn send_message(&self, message: &str) -> Result<String> {
        match self.provider.as_str() {
            "openai" => self.send_openai_message(message).await,
            "anthropic" => self.send_anthropic_message(message).await,
            _ => Err(anyhow::anyhow!("Unsupported provider: {}", self.provider)),
        }
    }
    
    async fn send_openai_message(&self, message: &str) -> Result<String> {
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![OpenAIMessage {
                role: "user".to_string(),
                content: message.to_string(),
            }],
            max_tokens: Some(1000),
        };
        
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("API Error: {}", error_text));
        }
        
        let openai_response: OpenAIResponse = response.json().await?;
        
        Ok(openai_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_else(|| "No response".to_string()))
    }
    
    async fn send_anthropic_message(&self, message: &str) -> Result<String> {
        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 1000,
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: message.to_string(),
            }],
        };
        
        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("API Error: {}", error_text));
        }
        
        let anthropic_response: AnthropicResponse = response.json().await?;
        
        Ok(anthropic_response
            .content
            .first()
            .map(|content| content.text.clone())
            .unwrap_or_else(|| "No response".to_string()))
    }
}
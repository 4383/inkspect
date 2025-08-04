use super::r#trait::LlmBackend;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct ClaudeBackend {
    api_key: String,
    client: Client,
    url: String,
    model: String,
}

#[derive(Serialize)]
struct ClaudeRequest {
    prompt: String,
    model: String,
    max_tokens_to_sample: u32,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    completion: String,
}

impl ClaudeBackend {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            url: "https://api.anthropic.com".to_string(),
            model,
        }
    }

    #[cfg(test)]
    pub fn new_with_url(api_key: String, url: String, model: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            url,
            model,
        }
    }
}

#[async_trait::async_trait]
impl LlmBackend for ClaudeBackend {
    async fn request(&self, full_prompt: &str) -> Result<String> {
        let full_url = format!("{}/v1/complete", self.url);

        let request_body = ClaudeRequest {
            prompt: full_prompt.to_string(),
            model: self.model.clone(),
            max_tokens_to_sample: 300,
        };

        let response = self
            .client
            .post(&full_url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("User-Agent", "inkspect/0.1.0")
            .json(&request_body)
            .send()
            .await?;
        let response_text = response.text().await?;
        if response_text.is_empty() {
            return Err(anyhow::anyhow!("Empty response from Claude API"));
        }
        log::debug!("Claude API response: {}", response_text);

        let json_value: serde_json::Value = serde_json::from_str(&response_text)?;

        if let Some(error) = json_value.get("error") {
            if let Some(message) = error.get("message") {
                return Err(anyhow::anyhow!(
                    "Claude API Error: {}",
                    message.as_str().unwrap_or("Unknown error")
                ));
            }
        }

        let claude_response: ClaudeResponse = serde_json::from_value(json_value)?;
        Ok(claude_response.completion)
    }

    async fn list_models(&self) -> Result<Vec<String>> {
        Ok(vec!["claude-2".to_string(), "claude-3".to_string()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_claude_backend_request() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/complete")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"completion":"Mocked Claude response"}"#)
            .create_async()
            .await;

        let backend = ClaudeBackend::new_with_url(
            "test_api_key".to_string(),
            server.url(),
            "claude-2".to_string(),
        );
        let response = backend.request("test prompt").await.unwrap();
        assert_eq!(response, "Mocked Claude response");
        mock.assert_async().await;
    }
}

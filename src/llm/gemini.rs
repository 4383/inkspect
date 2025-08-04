use super::r#trait::LlmBackend;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct GeminiBackend {
    api_key: String,
    client: Client,
    url: String,
    model: String,
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    safety_settings: Vec<SafetySetting>,
}

#[derive(Serialize)]
struct SafetySetting {
    category: String,
    threshold: String,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Deserialize)]
struct ContentResponse {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize)]
struct PartResponse {
    text: String,
}

impl GeminiBackend {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            url: "https://generativelanguage.googleapis.com".to_string(),
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
impl LlmBackend for GeminiBackend {
    async fn request(&self, full_prompt: &str) -> Result<String> {
        let full_url = format!(
            "{}/v1beta/{}:generateContent?key={}",
            self.url, self.model, self.api_key
        );

        let request_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: full_prompt.to_string(),
                }],
            }],
            safety_settings: vec![
                SafetySetting {
                    category: "HARM_CATEGORY_HARASSMENT".to_string(),
                    threshold: "BLOCK_NONE".to_string(),
                },
                SafetySetting {
                    category: "HARM_CATEGORY_HATE_SPEECH".to_string(),
                    threshold: "BLOCK_NONE".to_string(),
                },
                SafetySetting {
                    category: "HARM_CATEGORY_SEXUALLY_EXPLICIT".to_string(),
                    threshold: "BLOCK_NONE".to_string(),
                },
                SafetySetting {
                    category: "HARM_CATEGORY_DANGEROUS_CONTENT".to_string(),
                    threshold: "BLOCK_NONE".to_string(),
                },
            ],
        };

        let response = self
            .client
            .post(&full_url)
            .header("User-Agent", "inkspect/0.1.0")
            .json(&request_body)
            .send()
            .await?;
        let response_text = response.text().await?;
        if response_text.is_empty() {
            return Err(anyhow::anyhow!("Empty response from Gemini API"));
        }
        log::debug!("Gemini API response: {}", response_text);

        let json_value: serde_json::Value = serde_json::from_str(&response_text)?;

        if let Some(error) = json_value.get("error") {
            if let Some(message) = error.get("message") {
                return Err(anyhow::anyhow!(
                    "Gemini API Error: {}",
                    message.as_str().unwrap_or("Unknown error")
                ));
            }
        }

        let gemini_response: GeminiResponse = serde_json::from_value(json_value)?;
        Ok(gemini_response.candidates[0].content.parts[0].text.clone())
    }

    async fn list_models(&self) -> Result<Vec<String>> {
        let full_url = format!("{}/v1/models?key={}", self.url, self.api_key);
        let response = self.client.get(&full_url).send().await?;
        let response_text = response.text().await?;
        log::debug!("Gemini API response: {}", response_text);
        let models_response: ModelsResponse = serde_json::from_str(&response_text)?;
        Ok(models_response.models.into_iter().map(|m| m.name).collect())
    }
}

#[derive(Deserialize)]
struct ModelsResponse {
    models: Vec<Model>,
}

#[derive(Deserialize)]
struct Model {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_gemini_backend_request() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock(
                "POST",
                "/v1beta/gemini-2.5-pro:generateContent?key=test_api_key",
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"candidates":[{"content":{"parts":[{"text":"Mocked Gemini response"}]}}]}"#,
            )
            .create_async()
            .await;

        let backend = GeminiBackend::new_with_url(
            "test_api_key".to_string(),
            server.url(),
            "gemini-2.5-pro".to_string(),
        );
        let response = backend.request("test prompt").await.unwrap();
        assert_eq!(response, "Mocked Gemini response");
        mock.assert_async().await;
    }
}

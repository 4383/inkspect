use anyhow::Result;

#[async_trait::async_trait]
pub trait LlmBackend {
    async fn request(&self, full_prompt: &str) -> Result<String>;
    async fn list_models(&self) -> Result<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockLlmBackend;

    #[async_trait::async_trait]
    impl LlmBackend for MockLlmBackend {
        async fn request(&self, full_prompt: &str) -> Result<String> {
            Ok(format!("Mocked response for prompt: '{}'", full_prompt))
        }
        async fn list_models(&self) -> Result<Vec<String>> {
            Ok(vec!["model1".to_string(), "model2".to_string()])
        }
    }

    #[tokio::test]
    async fn test_mock_llm_backend() {
        let backend = MockLlmBackend;
        let response = backend.request("test prompt").await.unwrap();
        assert_eq!(response, "Mocked response for prompt: 'test prompt'");
    }
}

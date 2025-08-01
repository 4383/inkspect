use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub llm: Llm,
    pub providers: Providers,
    pub prompts: HashMap<String, String>,
}

impl Config {
    pub fn sanitized(&self) -> Self {
        let mut sanitized_config = self.clone();
        sanitized_config.providers.gemini.api_key = "[REDACTED]".to_string();
        sanitized_config.providers.claude.api_key = "[REDACTED]".to_string();
        sanitized_config
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Llm {
    pub provider: String,
    pub default_prompt: String,
    pub system_prompt: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Providers {
    pub gemini: Provider,
    pub claude: Provider,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Provider {
    pub api_key: String,
    pub model: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            llm: Llm {
                provider: "gemini".to_string(),
                default_prompt: "refine".to_string(),
                system_prompt: Some("You are an expert providing a direct and comprehensive answer. Your response should be direct, containing only the answer itself without any introductory remarks, conversational filler, or concluding statements. Do not add a summary or any closing comments. Get straight to the point.".to_string()),
            },
            providers: Providers {
                gemini: Provider {
                    api_key: "GEMINI_API_KEY".to_string(),
                    model: "models/gemini-2.5-pro".to_string(),
                },
                claude: Provider {
                    api_key: "CLAUDE_API_KEY".to_string(),
                    model: "claude-2".to_string(),
                },
            },
            prompts: {
                let mut map = HashMap::new();
                map.insert(
                    "refine".to_string(),
                    "You are an expert providing a direct and comprehensive answer. Respond clearly and specifically. Do not include any conversational filler, introductory remarks, or concluding statements. Get straight to the point. Your response should start directly with the answer, without any preamble.".to_string(),
                );
                map.insert(
                    "simplify".to_string(),
                    "You are an expert providing a concise and easy-to-understand answer. Explain the topic simply. Do not use conversational language or introductory phrases. Be direct.".to_string(),
                );
                map.insert(
                    "boost_engagement".to_string(),
                    "You are a social media expert. Generate an engaging post based on the input. The tone should be exciting and captivating. Do not add any extra conversational text. Output only the post content.".to_string(),
                );
                map.insert(
                    "code-agent-spec".to_string(),
                    "You are a senior software architect. Your task is to create a detailed specification for an AI coding agent. Do not write any code. Your output must be a Markdown document that guides the agent. The specification must enforce a strict Test-Driven Development (TDD) methodology. The document must include: 1. High-Level Goal, 2. Key Features, 3. Proposed Architecture & File Structure, 4. Data Structures & Types, 5. Step-by-Step TDD Implementation Plan (for each feature, specify the failing test to write first, then the implementation), 6. Error Handling, and 7. Testing Strategy (emphasizing unit tests for every feature). Your sole output is this specification document. Do not, under any circumstances, write the implementation code for the project. Your response must not contain any code.".to_string(),
                );
                map.insert(
                    "code-gen".to_string(),
                    "You are an expert AI programmer. Your task is to generate a complete, production-quality, single-file application based on the user's request. The code must be well-commented, robust, and follow best practices. Include a section on how to build and run the application. Your output should be a single Markdown file containing the code and instructions.".to_string(),
                );
                map
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[derive(serde::Serialize)]
    struct TestConfig {
        llm: TestLlmConfig,
        providers: TestProvidersConfig,
        prompts: HashMap<String, String>,
    }

    #[derive(serde::Serialize)]
    struct TestLlmConfig {
        provider: String,
        default_prompt: String,
    }

    #[derive(serde::Serialize)]
    struct TestProvidersConfig {
        gemini: TestProvider,
        claude: TestProvider,
    }

    #[derive(serde::Serialize)]
    struct TestProvider {
        api_key: String,
    }

    #[test]
    fn test_load_config() {
        let mut file = NamedTempFile::new().unwrap();
        let config = TestConfig {
            llm: TestLlmConfig {
                provider: "gemini".to_string(),
                default_prompt: "refine".to_string(),
            },
            providers: TestProvidersConfig {
                gemini: TestProvider {
                    api_key: "GEMINI_API_KEY".to_string(),
                },
                claude: TestProvider {
                    api_key: "CLAUDE_API_KEY".to_string(),
                },
            },
            prompts: {
                let mut map = HashMap::new();
                map.insert("refine".to_string(), "Refine this prompt".to_string());
                map
            },
        };
        let toml = toml::to_string(&config).unwrap();
        file.write_all(toml.as_bytes()).unwrap();

        let loaded_config: Config = confy::load_path(file.path()).unwrap();
        assert_eq!(loaded_config.llm.provider, "gemini");
        assert_eq!(loaded_config.llm.default_prompt, "refine");
        assert_eq!(loaded_config.providers.gemini.api_key, "GEMINI_API_KEY");
        assert_eq!(loaded_config.providers.claude.api_key, "CLAUDE_API_KEY");
        assert_eq!(
            loaded_config.prompts.get("refine"),
            Some(&"Refine this prompt".to_string())
        );
    }

    #[test]
    fn test_fallback_to_default() {
        let path = "non_existent_config_file.toml";
        let config: Config = confy::load_path(path).unwrap();
        assert_eq!(config.llm.provider, "gemini");
        assert_eq!(config.llm.default_prompt, "refine");
    }
}

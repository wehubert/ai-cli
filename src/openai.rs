use crate::config::{Assistant, Config};
use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};

#[derive(Debug, Clone)]
pub struct OpenAI {
    client: Client<OpenAIConfig>,
    current_assistant: String,
    assistants: Vec<Assistant>,
}

impl OpenAI {
    pub fn new(config: &Config) -> Self {
        let client: Client<OpenAIConfig> = Client::with_config(
            config
                .openai_api_key
                .clone()
                .map_or(OpenAIConfig::new(), |key| {
                    OpenAIConfig::new().with_api_key(key)
                }),
        );
        OpenAI {
            client,
            current_assistant: config
                .assistants
                .first()
                .map_or(String::new(), |assistant| assistant.name.to_string()),
            assistants: config.assistants.clone(),
        }
    }

    pub fn set_assistant(mut self, assistant: &str) -> Self {
        self.current_assistant = assistant.to_string();
        self
    }

    pub async fn chat(&self, message: &str) -> Result<String, OpenAIError> {
        let assistant = self
            .assistants
            .iter()
            .find(|assistant| assistant.name == self.current_assistant)
            .cloned()
            .unwrap_or_default();
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(assistant.max_tokens)
            .model(assistant.model)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(assistant.system_message)
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(message)
                    .build()?
                    .into(),
            ])
            .build()?;

        log::debug!("sending request: {:?}", &request);

        let response = self.client.chat().create(request).await?;

        log::debug!("response received: {:?}", &response);

        Ok(response.choices[0]
            .message
            .content
            .clone()
            .unwrap_or("No response".to_string()))
    }
}

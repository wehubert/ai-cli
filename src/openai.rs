use crate::config::Item;
use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};

pub async fn chat_openai(
    openai_api_key: Option<String>,
    message: String,
    config_item: Item,
) -> Result<String, OpenAIError> {
    let client: Client<OpenAIConfig> =
        Client::with_config(openai_api_key.map_or(OpenAIConfig::new(), |key| {
            OpenAIConfig::new().with_api_key(key)
        }));

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(config_item.max_tokens)
        .model(config_item.model)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(config_item.system_message)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(message)
                .build()?
                .into(),
        ])
        .build()?;

    log::debug!("sending request: {:?}", &request);

    let response = client.chat().create(request).await?;

    log::debug!("response received: {:?}", &response);

    Ok(response.choices[0]
        .message
        .content
        .clone()
        .unwrap_or("No response".to_string()))
}

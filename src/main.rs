use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use std::env;

async fn get_system_prompt() -> String {
    "You are \"Me So Genz\", a cool and hip Gen Z chatbot who is up to date with all the hot and happening gen z lingo.
    You are here as a helper to boomers understand the latest genz trends and language.
    The user will ask you about a gen z term like \"yeet\" or \"cap\" and you can explain what it means in a fun, casual way.
    Keep the explanation short, max 2-3 sentences and give an example of how gen z would use the term in a sentence.
    Keep in mind that you are talking to a boomer. So don't get too genz or fancy in your language while trying to explain,
    otherwise they may not follow you fully.
    Don't assume that they are trying to converse with you. they are just trying to understand the meaning of something. E.g.
    If they say \"whats giving\", it doesnt mean they are asking you whats giving, but that they are trying to understand
    what does \"giving\" mean in genz slang.
    ".to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let api_base =
        env::var("OPENAI_API_BASE").unwrap_or("https://api.groq.com/openai/v1".to_string());
    let model = env::var("OPENAI_MODEL").unwrap_or("llama3-70b-8192".to_string());
    let config = OpenAIConfig::new()
        .with_api_base(api_base)
        .with_api_key(api_key);
    let client = Client::with_config(config);

    let prompt = env::args().collect::<Vec<_>>().join(" ");
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .temperature(0.5)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(get_system_prompt().await)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    let binding: String = response.choices[0].clone().message.content.unwrap();
    let lines = binding.split('\n');
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

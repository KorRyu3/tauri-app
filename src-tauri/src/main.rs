// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate dotenv;
extern crate async_openai;
extern crate tokio;

use std::env;

use async_openai::{
    Client,
    config::AzureConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs
    }
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // tauri::Builder::default()
    //   .run(tauri::generate_context!())
    //   .expect("error while running tauri application");

    // 環境変数を読み込む
    dotenv().ok();

    // AzureOpenAIの設定
    let config = AzureConfig::new()
    .with_api_base(env::var("AZURE_OPENAI_API_ENDPOINT").unwrap())
    .with_api_version("2024-02-01")
    .with_deployment_id(env::var("AZURE_OPENAI_API_GPT_DEPLOYMENT").unwrap())
    .with_api_key(env::var("AZURE_OPENAI_API_KEY").unwrap());

    let client = Client::with_config(config);

    // userの入力を受け取る
    let mut input = String::new();
    println!("Please enter your message: ");
    std::io::stdin().read_line(&mut input).unwrap();

    let request = CreateChatCompletionRequestArgs::default()
    .model(env::var("AZURE_OPENAI_API_GPT_DEPLOYMENT").unwrap())
    .messages([
        ChatCompletionRequestSystemMessageArgs::default()
            .content("You are a helpful assistant.")
            .build()
            .unwrap()
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(input.trim())
            .build()
            .unwrap()
            .into(),
    ])
    .max_tokens(1024_u32)
    .build()
    .unwrap();

    let response = client
        .chat()
        .create(request)
        .await
        .unwrap();

    let response_text = response.choices[0].message.content.as_ref().unwrap();

    println!("reponse: {}", response_text);
}

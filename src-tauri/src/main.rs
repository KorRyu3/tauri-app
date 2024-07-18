// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Commandについて
// https://tauri.app/v1/guides/features/command/

extern crate dotenv;
extern crate async_openai;
extern crate tokio;
extern crate tauri;

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


fn main() {
    // 環境変数を読み込む
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // commandで定義した関数を入れる
            generate_response
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}


#[tauri::command]
async fn generate_response(input: String) -> String {

    // AzureOpenAIの設定
    let config = AzureConfig::new()
        .with_api_base(env::var("AZURE_OPENAI_API_ENDPOINT").unwrap())
        .with_api_version("2024-02-01")
        .with_deployment_id(env::var("AZURE_OPENAI_API_GPT_DEPLOYMENT").unwrap())
        .with_api_key(env::var("AZURE_OPENAI_API_KEY").unwrap());

    let client = Client::with_config(config);

    // system prompt
    const SYSTEM_PROMPT: &str = "You are a helpful assistant.";

    let request = CreateChatCompletionRequestArgs::default()
    .model(env::var("AZURE_OPENAI_API_GPT_DEPLOYMENT").unwrap())
    .messages([
        ChatCompletionRequestSystemMessageArgs::default()
            .content(SYSTEM_PROMPT)
            .build()
            .unwrap()
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(input)
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

    // unwrapを使用する際に所有権が渡ってしまうため、cloneを使用してコピーを作成する
    // もしくは、as_refを使用して参照を渡してもよい
    response.choices[0].message.content.clone().unwrap()

}

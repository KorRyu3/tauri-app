// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Commandについて
// https://tauri.app/v1/guides/features/command/

extern crate async_openai;
extern crate dotenv;
extern crate tauri;
extern crate tokio;

use std::env;
use std::sync::OnceLock;

use async_openai::{
    config::AzureConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, CreateChatCompletionRequest
    },
    Client,
};
use dotenv::dotenv;


// グローバル変数を扱いやすくする
static SYSTEM_PROMPT: OnceLock<String> = OnceLock::new();


#[tokio::main]
async fn main() {
    // 環境変数を読み込む
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // commandで定義した関数を入れる
            set_system_prompt,  // 一度だけ呼ばれる
            generate_response
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
async fn set_system_prompt(main_topic: String) {
    let system_prompt = format!(
        "これから、入力として会議中の会話が与えられます。この会話の内容が、会議の本筋から逸脱(脱線)しているかどうかを判断しなさい。
        判定の出力形式は下記の通りです。

        - 会議の本筋から逸脱していない場合: continue
        - 会議の本筋から逸脱している場合: deviation

        ## 例
        ### 会議の本筋から逸脱していない場合
        #### 入力
        A: 今日の天気は晴れですね。
        B: はい、そうですね。
        #### 出力
        continue

        ### 会議の本筋から逸脱している場合
        #### 入力
        A: 今日の天気は晴れですね。
        B: はい、そうですね。そういえば、昨日の夜、新しい映画を見ました。
        #### 出力
        deviation

        ## 会議の本筋
        {main_topic}",
        main_topic=main_topic
    );

    SYSTEM_PROMPT.set(system_prompt).unwrap();
}


#[tauri::command]
async fn generate_response(input: String) -> String {
    let config = AzureConfig::new()
        .with_api_base(env::var("AZURE_OPENAI_API_ENDPOINT").unwrap())
        .with_api_version("2024-02-01")
        .with_deployment_id(env::var("AZURE_OPENAI_API_GPT_DEPLOYMENT").unwrap())
        .with_api_key(env::var("AZURE_OPENAI_API_KEY").unwrap());

    let client = Client::with_config(config);

    let system_prompt = SYSTEM_PROMPT.get().unwrap().clone();

    let request = create_completion_request(input, system_prompt);

    let response = client.chat().create(request).await.unwrap();

    // clone/as_refを付けないと、Vec<ChatChoice>の要素が無効化されるエラー?が起きる可能性があるからコンパイルエラーになるらしい
    response.choices[0].message.content.clone().unwrap()
}


fn create_completion_request(input: String, system_prompt: String) -> CreateChatCompletionRequest {
    CreateChatCompletionRequestArgs::default()
    .model(env::var("AZURE_OPENAI_API_GPT_DEPLOYMENT").unwrap())
    .messages([
        ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
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
    .unwrap()
}

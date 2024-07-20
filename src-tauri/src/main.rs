// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Commandについて
// https://tauri.app/v1/guides/features/command/

extern crate async_openai;
extern crate dotenv;
extern crate tauri;
extern crate tokio;

use std::env;

use async_openai::{
    config::AzureConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use dotenv::dotenv;

// system prompt
const SYSTEM_PROMPT: &str = "
これから、入力として会議中の会話が与えられます。この会話の内容が、会議の本筋から逸脱(脱線)しているかどうかを判断しなさい。
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
{今日のご飯について}
";

#[tokio::main]
async fn main() {
    // 環境変数を読み込む
    dotenv().ok();

    // コンソールからGPTとの対話を行う時はコメントアウト
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // commandで定義した関数を入れる
            generate_response
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // コンソールからGPTとの対話を行う
    // 実行コマンド
    // cd src-tauri && cargo run
    // 下記はコメントアウト
    // let mut input = String::new();
    // println!("Please input your message: ");
    // std::io::stdin().read_line(&mut input).unwrap();
    // println!("response: {}", generate_response(input).await);
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

    let response = client.chat().create(request).await.unwrap();

    // unwrapを使用する際に所有権が渡ってしまうため、cloneを使用してコピーを作成する
    // もしくは、as_refを使用して参照を渡してもよい
    response.choices[0].message.content.clone().unwrap()
}

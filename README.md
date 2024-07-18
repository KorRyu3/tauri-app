This is a [Next.js](https://nextjs.org/) project bootstrapped with [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app).

## Getting Started

First, run the development server:

```bash
cargo tauri dev
# or
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/basic-features/font-optimization) to automatically optimize and load Inter, a custom Google Font.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js/) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/deployment) for more details.

## Setup

```bash
git clone git@github.com:KorRyu3/tauri-app.git
cd tauri-app
```

### rustupをインストール

Rust公式のツールチェーン管理ツール

インストールオプションが出たら、「**1**」を選択してください

#### MacOS

Homebrewを使ってインストール。その他の方法は、[公式サイト](https://www.rust-lang.org/ja/tools/install)を参照してください

```bash
# rustupをインストール
brew install rustup-init
rustup-init

# rustupの補完を設定
mkdir ~/.zfunc
rustup completions zsh > ~/.zfunc/_rustup
echo "fpath+=~/.zfunc" >> ~/.zshrc
```

#### Windows

```bash
# rustupをインストール
# 公式インストーラーを使いインストール
# https://www.rust-lang.org/ja/tools/install

# rustupの補完を設定
rustup completions powershell >> $PROFILE.CurrentUserCurrentHost
```

### voltaをインストール

Nodeのランタイムバージョン管理ツール

#### MacOS

```bash
# mac
brew install volta
```

#### Windows

公式インストーラーを使いインストール

[公式サイト](https://docs.volta.sh/guide/getting-started)の「Windows Installation」を参考に

### Node, corepack, pnpmをインストール

ここからのコマンドは、必ずトップディレクトリで実行してください

```bash
node -v
volta install corepack@0.28.2

# Yを選択
pnpm -v

# pnpmでnodeのパッケージをインストール
pnpm install --frozen-lockfile
```

### 起動にcargoを使用する場合

※ cargoを使って起動しない場合は、この手順は不要です

```bash
# tauri-cliをグローバルにインストール
# 多少時間がかかります
cargo install tauri-cli --version 1.6.0
```

## プロジェクトの起動

```bash
# pnpmで起動
pnpm tauri dev

# cargoで起動
cargo tauri dev
```

## TODO

- [x] windowsのrustupのインストール手順を追記

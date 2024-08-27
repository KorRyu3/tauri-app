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

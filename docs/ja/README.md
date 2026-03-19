# linkura-cli

[English](../../README.md) | [简体中文](../zh-CN/README.md)

現在のリポジトリには主に以下の 2 つのプログラムが含まれます。
- `linkura-cli`
- `linkura-motion-cli`

## リポジトリの機能

### linkura-cli

1. 最新の配信情報と最新のアーカイブ情報を取得できます。
2. 指定したアーカイブの詳細情報を取得できます。
3. MCP サーバーを起動できます。

#### MCP サーバー機能

##### tools

- `list_live_streaming_info`: 現在の配信予告情報を一覧表示します
- `list_archives`: アーカイブ情報を一覧表示します
- `get_archive_detail`: 特定のアーカイブの詳細情報を取得します

### linkura-motion-cli

1. `iarc` / `md` 形式のモーションキャプチャデータを**マルチスレッドでダウンロード**できます
2. モーションキャプチャデータを指定した S3 互換ストレージサーバーへ**マルチスレッドでアップロード**できます
3. モーションキャプチャパケットを**解析**し、解析結果を出力できます
4. モーションキャプチャパケット内の音声を**抽出**して opus 形式で出力できます
5. **ライブ**のモーションキャプチャパケットを**アーカイブ**用パケットへ**変換**できます

## リポジトリ構成

- `bin/linkura-cli`
  - メイン CLI プログラム
  - MCP サービス実装を含みます
- `bin/linkura-motion-cli`
  - モーションキャプチャデータ関連ツール
- `crates/api`
  - 高レベルおよび低レベル API ラッパー
- `crates/packet`
  - モーションキャプチャプロトコルとパケット処理
- `crates/downloader`
  - モーションキャプチャデータのダウンロード / アップロード関連ロジック
- `crates/common`
  - 共通ヘルパー関数および型
- `crates/i18n`
  - 多言語国際化サポート
- `deps/`
  - ビルド時コード生成に必要な外部メタデータ

## ビルド

### 依存関係

- このワークスペースに対応した Rust ツールチェーン
- `protoc`
- `cmake`
- `audio` feature を有効にした `linkura-motion-cli` をビルドする場合は `libopus` が必要です。詳細な依存条件は [opusic-sys](https://github.com/DoumanAsh/opusic-sys/) を参照してください

### 手順

```bash
git clone https://github.com/ChocoLZS/linkura-cli.git
cd linkura-cli
cargo build -p linkura-cli
cargo build -p linkura-motion-cli
```

例:

```bash
cargo run -p linkura-cli
```

```bash
cargo run -p linkura-motion-cli
```

```bash
cargo run -p linkura-motion-cli --features audio
```

## 免責事項

- 本リポジトリは非公式プロジェクトであり、ゲーム運営会社および関連権利者とは一切関係ありません。
- リポジトリの内容は主に個人学習、研究、およびツール開発のために提供されています。
- 正確性、完全性、長期的な互換性、または特定用途への適合性について、作者は一切保証しません。
- 利用者は、自身の利用方法が現地の法令、プラットフォーム規約、および第三者の権利要件に適合していることを自ら確認する必要があります。

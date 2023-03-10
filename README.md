# YouTube ライブ配信のチャットを読み上げるツール

## CLI 版の起動方法

### 環境変数

- `VIDEO_ID` : チャット欄を読み上げたいライブ配信の ID (URL 末尾 `?v=xxx` の `xxx` の部分)
- `YOUTUBE_API_KEY` : Google Cloud Platform の管理画面で取得した、YouTube Data API v3 の API キー

GUI 版の VOICEVOX を起動したうえで、以下のコマンドを実行してください。

```bash
$ cargo run --bin yt-chat-tts-cli
```

## GUI 版の起動方法

[asdf](https://asdf-vm.com/) がインストールされていることを前提としています。

```bash
asdf install
cd gui
pnpm install
pnpm tauri dev
```

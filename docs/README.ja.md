<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">待機中の負荷がゼロのスクリーンショットツール（Windows、macOS、Linux）</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · **日本語** · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md)

![サンプル画面上のエディター：マーカー、枠、矢印、文字](editor.png)

Print Screen を押し、範囲を選び、書き込んでから、画像をコピー、保存、アップロードします。撮影の合間はキーを待つだけで、ウィンドウもタイマーもなく、プロセッサーを使いません。

## できること

- すべてのモニターを撮影し、その上にエディターを開きます。
- 鉛筆、直線、矢印、枠、マーカー、文字を 8 色で書き込めます。
- 画像をコピーするか、PNG で保存するか、[自分のサーバー](../server/README.md)にアップロードしてリンクをコピーします。
- 12 の言語に対応しています。最初はシステムの言語で、メニューから変更できます。
- 自分からは何も送信しません。アカウント、テレメトリー、更新確認はありません。ネットワークは、設定したサーバーに画像をアップロードするときだけ使います。

## エディターのキー

| キー | 動作 |
| --- | --- |
| Ctrl+C | 範囲をコピー |
| Ctrl+S | 範囲を`ピクチャ/Screenshots`に保存 |
| Ctrl+U | 範囲をアップロードし、リンクをクリップボードへ |
| Ctrl+Z | 直前の書き込みを取り消し |
| Esc | エディターを閉じる |

macOS では Ctrl の代わりに ⌘ を使います。

トレイアイコンのメニューに、ショートカット、フォルダー、言語、サーバーのアドレス、システム起動時の開始があります。プログラムをもう一度起動しても撮影します。

## 軽さ

2026-11-24、Windows Server 2022、1920 × 1080 の画面、リリースビルドで測定：

| 項目 | 結果 |
| --- | --- |
| 待機中のプロセッサー時間 | 10 秒間で 0 ミリ秒 |
| 待機中のメモリー | 最初の撮影前 14 MB、7 回後 25 MB |
| 待機中のプロセス数 | 1 |
| キーを押してからエディターが出るまで | 7 回の測定で 0.61～0.75 秒 |
| プログラムの大きさ | 5.0 MB |

エディターが開くまでにこれだけかかるのは、撮影のたびにウィンドウを作り、終わると破棄するためです。待機中の負荷をゼロにする代償です。

## 状況

| システム | 状況 |
| --- | --- |
| Windows | Windows Server 2022 とモニター 1 台で動作します。Windows 10 と 11 ではまだ確認していません。 |
| macOS 10.13+ | ビルドとテストは通ります。まだ誰も実行していません。 |
| Ubuntu 24.04+ | ビルドとテストは通ります。まだ誰も実行していません。Wayland ではキーボードの設定でショートカットを割り当ててください。コマンドは `just-screenshotter` です。 |

## インストール

ダウンロードできるファイルはまだないため、ビルドが必要です。Node.js 24 以降と Rust、Windows ではさらに Microsoft C++ Build Tools、Ubuntu ではさらに `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev` が必要です。

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

インストーラーは `src-tauri/target/release/bundle` にできます。署名がないため、Windows と macOS は信頼するかどうかを尋ねます。

## 翻訳

翻訳は機械によるもので、母語話者による確認は受けていません。間違いを見つけたら `locales` または `docs` を直してください。

## 参加とライセンス

ビルド、テスト、変更の方法：[CONTRIBUTING.md](../CONTRIBUTING.md)。ライセンスは [MIT](../LICENSE) です。

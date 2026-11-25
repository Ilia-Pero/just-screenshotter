<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">待機中の負荷がゼロのスクリーンショットツール（Windows、macOS、Linux）</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · **日本語** · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md)

![サンプル画面上のエディター：マーカー、枠、矢印、文字](editor.png)

Print Screen を押し、範囲を選び、書き込んでから、画像をコピー、保存、アップロードします。範囲を動画として録画することもできます。撮影の合間はキーを待つだけで、ウィンドウもタイマーもなく、プロセッサーを使いません。

## できること

- すべてのモニターを撮影し、その上にエディターを開きます。
- 鉛筆、直線、矢印、枠、マーカー、文字を 8 色で書き込めます。
- 書き込んだものを後から変更できます。クリックで選び、本体や端をドラッグし、太さと色を変え、文字を打ち直し、削除できます。
- 画像をコピーするか、PNG で保存するか、[自分のサーバー](../server/README.md)にアップロードしてリンクをコピーします。
- 範囲を音声付きの動画として録画します。Windows は MP4、macOS は MOV、Linux は WebM です。パソコンの音とマイクを録音するかは選べます。macOS 13 より前ではマイクだけです。録画は「停止」ボタン、ショートカット、トレイアイコンで終わります。
- 12 の言語に対応しています。言語はエディター、設定、Windows のインストーラーで選べます。選ぶまではシステムの言語です。
- 自分からは何も送信しません。アカウント、テレメトリー、更新確認はありません。ネットワークは、設定したサーバーに画像をアップロードするときだけ使います。

## キーとマウス

| 動作 | Windows と Linux | macOS |
| --- | --- | --- |
| 範囲のスクリーンショット | Print Screen | ⌘⇧9 |
| 画面全体のスクリーンショット | Shift + Print Screen | — |
| 使用中のウィンドウのスクリーンショット (Windows) | Alt + Print Screen | — |
| 範囲の動画 | Ctrl + Shift + Print Screen | ⌘⇧8 |

ほかのキーは設定で選べます。Windows ではマウスのボタンでも同じことができます。左右同時に、そのまま、または Shift や Ctrl と一緒に押します。はじめはオフです。ボタンを見張っている間は、マウスを動かすたびに約 2 µs のプロセッサー時間を使うためです。止まっていれば何も使いません。Wayland ではシステムの設定でキーを割り当てます。

## エディターのキー

| キー | 動作 |
| --- | --- |
| Ctrl+C | 範囲をコピー |
| Ctrl+S | 範囲を`ピクチャ/Screenshots`に保存 |
| Ctrl+U | 範囲をアップロードし、リンクをクリップボードへ |
| Ctrl+Z | 直前の書き込みを取り消し |
| Delete | 選んだ書き込みを削除 |
| Esc | エディターを閉じる |

macOS では Ctrl の代わりに ⌘ を使います。

道具を選んでいないときに書き込みを選べます。ダブルクリックか Enter で文字を変更し、ホイールで太さを変えます。

そのほかも設定にあります。動画と一緒に録音するもの、フォルダー、言語、アップロード、システム起動時の開始です。起動時の開始は、オフにするまでオンです。設定はトレイアイコンのメニューか、`--settings` を付けた起動で開きます。付けずにもう一度起動しても撮影します。

## 軽さ

2026-11-24、Windows Server 2022、1920 × 1080 の画面、リリースビルドで測定：

| 項目 | 結果 |
| --- | --- |
| 待機中のプロセッサー時間 | 10 秒間で 0 ミリ秒 |
| 待機中のメモリー | 最初の撮影前 14 MB、7 回後 25 MB |
| 待機中のプロセス数 | 1 |
| キーを押してからエディターが出るまで | 7 回の測定で 0.61～0.75 秒 |
| プログラムの大きさ | 5.2 MB |

エディターが開くまでにこれだけかかるのは、撮影のたびにウィンドウを作り、終わると破棄するためです。待機中の負荷をゼロにする代償です。

動画は別のプロセスが録画し、終わるとメモリーごと消えます。グラフィックカードのないこのサーバーでは、1920 × 1080 を毎秒 30 フレームで録ると 1 コアと 207 MB、800 × 600 では 0.4 コアと 78 MB を使います。H.264 をエンコードできるグラフィックカードがあれば、その仕事を引き受けます。これはまだ測っていません。

## 状況

| システム | 状況 |
| --- | --- |
| Windows | Windows Server 2022 とモニター 1 台で動作します。動画もそこで確認しました。音声デバイスからの録音は、サーバーにデバイスがないため確認していません。Windows 10 と 11 ではまだ確認していません。 |
| macOS 10.13+ | ビルドとテストは通り、GitHub のマシンでインストール、起動、パソコンの音を含む画面の録画ができます。まだ誰も手で試していません。 |
| Ubuntu 24.04+ | ビルドとテストは通り、GitHub のマシンでインストール、起動、画面の録画ができます。まだ誰も手で試していません。Wayland ではキーボードの設定でショートカットを割り当ててください。コマンドは `just-screenshotter` です。 |

## インストール

| システム | ファイル | インストール方法 |
| --- | --- | --- |
| Windows 10 と 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | 実行します。Windows が警告したら「詳細情報」、「実行」を選びます。 |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | プログラムを「アプリケーション」にドラッグして開きます。macOS に拒否されたら、システム設定の「プライバシーとセキュリティ」で「このまま開く」を押します。 |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |

GitHub はこのリポジトリのコードからこれらのファイルをビルドし、それぞれをインストールしてプログラムを起動し、そのうえで公開します。有料の署名がないため、Windows と macOS は初回起動時に警告します。

自分でビルドすることもできます。Node.js 24 以降と Rust、Windows ではさらに Microsoft C++ Build Tools、Ubuntu ではさらに `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev` が必要です。

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

インストーラーは `src-tauri/target/release/bundle` にできます。

## 翻訳

翻訳は機械によるもので、母語話者による確認は受けていません。間違いを見つけたら `locales` または `docs` を直してください。

## 参加とライセンス

ビルド、テスト、変更の方法：[CONTRIBUTING.md](../CONTRIBUTING.md)。ライセンスは [MIT](../LICENSE) です。

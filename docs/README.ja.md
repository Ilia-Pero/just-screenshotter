<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Windows、macOS、Linux 用のスクリーンショットと画面録画</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · **日本語** · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![サンプル画面の上のエディター：マーカー、楕円、矢印、文字](editor.png)

Print Screen を押すと、すべてのモニターを撮影し、その上にエディターを開きます。範囲を選び、描き込んで、画像をコピー、保存、送信できます。同じ範囲を動画として録画することもできます。撮影していない間は、トレイにアイコンを持つ 1 つのプロセスだけがメモリに残ります。

## インストール

| システム | ファイル | 方法 |
| --- | --- | --- |
| Windows 10, 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | ファイルを実行します。Windows の警告が出たら「詳細情報」、「実行」の順に選びます。 |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | プログラムを「アプリケーション」に移して開きます。macOS に拒否されたら、システム設定の「プライバシーとセキュリティ」で「このまま開く」を押します。 |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | `about:debugging` を開き、「この Firefox」、「一時的なアドオンを読み込む」でファイルを選びます。Firefox を再起動したら、もう一度読み込みます。 |
| Chrome, Edge, Opera, Yandex Browser | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | アーカイブを、削除しないフォルダーに展開します。`chrome://extensions` を開き、デベロッパーモードをオンにして「パッケージ化されていない拡張機能を読み込む」でフォルダーを選びます。 |

ファイルに有料の署名がないため、Windows と macOS は警告を表示します。GitHub がこのリポジトリからファイルを作成し、新しい環境にプログラムをインストールして起動した後に公開します。

## 機能

- 鉛筆、線、矢印、枠、楕円、マーカー、文字で描画。8 色、太さは 1～72 ピクセル。
- 描いたものはいつでも移動、色や大きさの変更、文字の入力し直し、削除ができます。
- 画像をコピー、PNG で保存、または[自分のサーバー](../server/README.md)に送信します。
- 範囲をパソコンの音とマイク付きで録画：Windows は MP4、macOS は MOV、Linux は WebM。macOS 13 より前はマイクのみ録音します。
- 14 言語に対応。キーと保存先フォルダーは設定で変更できます。

## キー

| 操作 | Windows, Linux | macOS |
| --- | --- | --- |
| 範囲のスクリーンショット | Print Screen | ⌘⇧9 |
| 画面全体のスクリーンショット | Shift + Print Screen | |
| アクティブウィンドウのスクリーンショット（Windows のみ） | Alt + Print Screen | |
| 範囲の動画 | Ctrl + Shift + Print Screen | ⌘⇧8 |

キーは設定で変更できます。エディターでは Ctrl+C がコピー、Ctrl+S が保存、Ctrl+Z が取り消し、Esc が終了です。

## ブラウザー拡張機能

ページの見えている部分（Alt+Shift+S）またはページ全体（Alt+Shift+P）を撮影し、同じエディターを新しいタブで開きます。Firefox と Chromium ベースのブラウザーで動作します。

## データとネットワーク

画像と動画はあなたのコンピューターに残ります。プログラムがネットワークを使うのは、自分のサーバーに画像を送るときと「更新を確認」を押したときです。アカウントとテレメトリーはありません。

## 詳細

測定結果、確認済みのシステム、ビルド方法は[英語版](../README.md)にあります。

この文章は機械翻訳で、母語話者による確認を受けていません。修正は `docs` フォルダーにお願いします。

[MIT](../LICENSE)

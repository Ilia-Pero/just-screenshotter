<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">适用于 Windows、macOS 和 Linux 的截图与录屏工具</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · **简体中文** · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![示例屏幕上的编辑器：荧光笔、椭圆、箭头和文字](editor.png)

按下 Print Screen 后，程序会截取所有显示器，并在屏幕上打开编辑器。选择区域、在上面绘制，然后复制、保存或上传图片。同一区域也可以录制成视频。不截图时，内存中只保留一个带托盘图标的进程。

## 安装

| 系统 | 文件 | 安装方法 |
| --- | --- | --- |
| Windows 10, 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | 运行文件。如果 Windows 显示警告，请选择“更多信息”，再选择“仍要运行”。 |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | 将程序拖到“应用程序”并打开。macOS 首次拒绝后，打开系统设置中的“隐私与安全性”，点击“仍要打开”。 |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | 打开 `about:debugging`，选择“此 Firefox”、“临时载入附加组件”，然后选择文件。重启 Firefox 后需要重新载入。 |
| Chrome, Edge, Opera, Yandex Browser | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | 将压缩包解压到会保留的文件夹。打开 `chrome://extensions`，开启开发者模式，点击“加载已解压的扩展程序”并选择该文件夹。 |

这些文件没有付费签名，因此 Windows 和 macOS 会显示警告。GitHub 从本仓库构建文件，在干净的机器上安装并启动程序，之后才发布版本。

## 功能

- 用铅笔、直线、箭头、方框、椭圆、荧光笔和文字绘制：八种颜色，粗细 1 到 72 像素。
- 已绘制的内容随时可以移动、改色、调整大小、重新输入文字或删除。
- 图片可以复制、保存为 PNG，或上传到[您自己的服务器](../server/README.md)。
- 录制区域视频，可包含电脑声音和麦克风：Windows 为 MP4，macOS 为 MOV，Linux 为 WebM。macOS 13 之前只录制麦克风。
- 14 种语言；快捷键和文件夹可在设置中更改。

## 快捷键

| 操作 | Windows, Linux | macOS |
| --- | --- | --- |
| 区域截图 | Print Screen | ⌘⇧9 |
| 全屏截图 | Shift + Print Screen | |
| 活动窗口截图（仅 Windows） | Alt + Print Screen | |
| 区域视频 | Ctrl + Shift + Print Screen | ⌘⇧8 |

快捷键可在设置中更改。在编辑器中，Ctrl+C 复制，Ctrl+S 保存，Ctrl+Z 撤销，Esc 关闭。

## 浏览器扩展

截取网页的可见部分（Alt+Shift+S）或整个网页（Alt+Shift+P），并在新标签页中打开同一编辑器。适用于 Firefox 和基于 Chromium 的浏览器。

## 数据与网络

截图和录像保存在您的电脑上。程序只在您把图片上传到自己的服务器，以及点击“检查更新”时使用网络。没有账号和遥测。

## 更多信息

测量数据、已测试的系统和构建方法见[英文文档](../README.md)。

本文为机器翻译，未经母语者校对。更正请提交到 `docs` 文件夹。

[MIT](../LICENSE)

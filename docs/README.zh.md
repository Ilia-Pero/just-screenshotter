<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">空闲时零负载的截图工具，支持 Windows、macOS 和 Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · **简体中文** · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md)

![示例屏幕上的编辑器：荧光笔、方框、箭头和文字](editor.png)

按下 Print Screen，选择区域，在上面绘制，然后复制、保存或上传图片，也可以把该区域录成视频。两次截图之间，程序只等待按键：没有窗口，没有定时器，不占用处理器。

## 功能

- 截取所有显示器，并在其上打开编辑器。
- 用铅笔、直线、箭头、方框、荧光笔和文字绘制，八种颜色。
- 可以修改已绘制的内容：点击选中一个标记，然后拖动它或它的端点，设置粗细和颜色，重新输入文字或删除。
- 复制图片、保存为 PNG，或上传到[自己的服务器](../server/README.md)并复制链接。
- 将区域录成带声音的视频：Windows 上为 MP4，macOS 上为 MOV，Linux 上为 WebM。是否录制电脑声音和麦克风由您选择；macOS 13 之前的版本只能录麦克风。用“停止”按钮、快捷键或托盘图标结束录制。
- 支持 12 种语言。可在编辑器、设置和 Windows 安装程序中选择语言；选择之前使用系统语言。
- 不会自行发送任何内容：没有账号，没有遥测，没有更新检查。仅在把图片上传到您设置的服务器时使用网络。

## 按键和鼠标

| 作用 | Windows 和 Linux | macOS |
| --- | --- | --- |
| 区域截图 | Print Screen | ⌘⇧9 |
| 全屏截图 | Shift + Print Screen | — |
| 活动窗口截图 (Windows) | Alt + Print Screen | — |
| 区域视频 | Ctrl + Shift + Print Screen | ⌘⇧8 |

其他按键可在设置中选择。在 Windows 上，鼠标键也能启动同样的操作：同时按下左右键，可单独按，也可与 Shift 或 Ctrl 一起按。默认关闭，因为程序监视鼠标键期间，鼠标每次移动约占用 2 µs 处理器时间；静止时不占用。在 Wayland 上，按键在系统设置中指定。

## 编辑器中的按键

| 按键 | 作用 |
| --- | --- |
| Ctrl+C | 复制区域 |
| Ctrl+S | 将区域保存到`图片/Screenshots` |
| Ctrl+U | 上传区域，链接进入剪贴板 |
| Ctrl+Z | 撤销上一次绘制 |
| Delete | 删除选中的标记 |
| Esc | 关闭编辑器 |

在 macOS 上用 ⌘ 代替 Ctrl。

未选择工具时可以选中标记。双击或按 Enter 修改文字。鼠标滚轮调整粗细。

其余也在设置中：录视频时录什么声音、文件夹、语言、上传，以及开机启动（默认开启，可关闭）。设置可从托盘图标的菜单打开，也可用 `--settings` 启动程序打开。不带它再次启动程序也会截图。

## Firefox 扩展

截取页面的可见部分或整个页面，并在新标签页中打开同样的编辑器：选择、绘制、复制、保存。按键为 Alt+Shift+S 和 Alt+Shift+P。只有在您要求时才查看页面，不发送任何内容，只记住语言。不录视频，截图高度最多 32,000 像素。

## 有多轻

2026-11-24 在 Windows Server 2022、1920 × 1080 屏幕上用发布版测得：

| 项目 | 结果 |
| --- | --- |
| 空闲时的处理器占用 | 10 秒内 0 毫秒 |
| 空闲时的内存 | 首次截图前 14 MB，七次后 25 MB |
| 空闲时的进程数 | 1 |
| 从按键到编辑器出现 | 七次测得 0.61–0.75 秒 |
| 程序大小 | 5.2 MB |

编辑器需要这么久才能打开，因为它的窗口在每次截图时创建，之后销毁。这是空闲时零负载的代价。

视频由单独的进程录制，录制结束后该进程退出并归还全部内存。在这台没有显卡的服务器上，1920 × 1080、每秒 30 帧占用一个处理器核心和 207 MB；800 × 600 占用 0.4 个核心和 78 MB。能编码 H.264 的显卡会接管这项工作，尚未测量。

## 状态

| 系统 | 状态 |
| --- | --- |
| Windows | 在 Windows Server 2022 上用一台显示器可以使用。视频也在那里检查过；未检查从设备录音，因为该服务器没有音频设备。尚未在 Windows 10 和 11 上检查。 |
| macOS 10.13+ | 可以构建并通过测试；在 GitHub 的机器上能安装、启动并录制屏幕和电脑声音。还没有人手动试过。 |
| Ubuntu 24.04+ | 可以构建并通过测试；在 GitHub 的机器上能安装、启动并录制屏幕。还没有人手动试过。在 Wayland 上，请在键盘设置中指定快捷键，命令为 `just-screenshotter`。 |
| Firefox 128+ | 在 Firefox 161 中可用，已在无窗口模式下检查。尚未经 Mozilla 签名，重启 Firefox 后需要重新加载。 |

## 安装

| 系统 | 文件 | 安装方法 |
| --- | --- | --- |
| Windows 10 和 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | 运行它。如果 Windows 不认识该程序：“更多信息”、“仍要运行”。 |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | 把程序拖到“应用程序”并打开。macOS 拒绝时，打开“系统设置”中的“隐私与安全性”，点按“仍要打开”。 |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | 打开 `about:debugging`，选择“此 Firefox”、“临时载入附加组件”和该文件。重启 Firefox 后需重新载入。 |

GitHub 用本仓库的代码构建这些文件，逐个安装并启动程序，然后才发布。它们没有付费签名，因此 Windows 和 macOS 会在首次启动时发出警告。

也可以自行构建程序。需要 Node.js 24 或更新版本和 Rust；Windows 还需要 Microsoft C++ Build Tools；Ubuntu 还需要 `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`。

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

安装程序位于 `src-tauri/target/release/bundle`。

## 翻译

译文由机器生成，未经母语者检查。如果发现错误，请在 `locales` 或 `docs` 中修正。

## 参与和许可证

如何构建、测试和修改程序：[CONTRIBUTING.md](../CONTRIBUTING.md)。许可证为 [MIT](../LICENSE)。

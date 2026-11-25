<p align="center"><img src="src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">A screenshot tool with zero idle load for Windows, macOS and Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

**English** · [Русский](docs/README.ru.md) · [Español](docs/README.es.md) · [Ελληνικά](docs/README.el.md) · [עברית](docs/README.he.md) · [简体中文](docs/README.zh.md) · [日本語](docs/README.ja.md) · [한국어](docs/README.ko.md) · [Српски](docs/README.sr.md) · [Français](docs/README.fr.md) · [Deutsch](docs/README.de.md) · [Italiano](docs/README.it.md)

![The editor over a sample screen: a marker, a frame, an arrow and a label](docs/editor.png)

Press Print Screen, select an area, draw on it, then copy, save or upload the picture, or record the area as video. Between screenshots the program only waits for the key: it has no windows, no timers, and takes no processor time.

## What it does

- Takes a screenshot of all monitors and opens the editor over them.
- Draws with a pencil, a line, an arrow, a frame, a marker and text in eight colors.
- Changes what is already drawn: a click picks a mark, and then you drag it or its ends, set its thickness and color, type its text anew or delete it.
- Copies the picture, saves it as PNG, or uploads it to [your own server](server/README.md) and copies the link.
- Records the area as video with sound: MP4 on Windows, MOV on macOS, WebM on Linux. You choose whether to record the sound of the computer and the microphone; on macOS before 13, only the microphone. The button Stop, the shortcut or the icon in the tray ends the recording.
- Speaks 12 languages. The language is chosen in the editor, in the settings and in the installer for Windows; until then it is the language of the system.
- Sends nothing anywhere by itself: there is no account, no telemetry and no update check. It uses the network only to upload a picture to the server you have set.

## Keys and mouse

| What it does | Windows and Linux | macOS |
| --- | --- | --- |
| Screenshot of an area | Print Screen | ⌘⇧9 |
| Screenshot of the whole screen | Shift + Print Screen | — |
| Screenshot of the active window (Windows) | Alt + Print Screen | — |
| Video of an area | Ctrl + Shift + Print Screen | ⌘⇧8 |

You can choose other keys in the settings. On Windows buttons of the mouse can start the same: both at once, alone, with Shift or with Ctrl. This is off at first, because while the program watches the buttons, every movement of the mouse takes about 2 µs of processor time; at rest it takes nothing. On Wayland the desktop assigns the key.

## Keys in the editor

| Key | What it does |
| --- | --- |
| Ctrl+C | copies the area |
| Ctrl+S | saves the area to `Pictures/Screenshots` |
| Ctrl+U | uploads the area, the link goes to the clipboard |
| Ctrl+Z | undoes the last drawing |
| Delete | deletes the picked mark |
| Esc | closes the editor |

On macOS, ⌘ takes the place of Ctrl.

Marks are picked when no tool is chosen. A double click or Enter changes a text. The mouse wheel changes the thickness.

The rest is in the settings too: what to record with video, the folder, the language, uploading and starting with the system, which is on until you turn it off. The settings are opened by the menu of the tray icon or by starting the program with `--settings`. Starting it for the second time without that also takes a screenshot.

## How light it is

Measured on 2026-11-24 on Windows Server 2022, a 1920 × 1080 screen, the release build:

| What | Result |
| --- | --- |
| Processor time at rest | 0 ms in 10 s |
| Memory at rest | 14 MB before the first screenshot, 25 MB after seven |
| Processes at rest | 1 |
| From the key press to the editor | 0.61–0.75 s in seven runs |
| Size of the program | 5.2 MB |

The editor takes this long to open because its window is created for every screenshot and destroyed after it. This is the price of zero load at rest.

Video is recorded by a separate process, which is gone with all its memory when the recording ends. On this server, which has no video card, 30 frames a second of 1920 × 1080 take one processor core and 207 MB, of 800 × 600 take 0.4 of a core and 78 MB. A video card that encodes H.264 takes this work over; that has not been measured yet.

## Status

| System | State |
| --- | --- |
| Windows | Works on Windows Server 2022 with one monitor. Video is checked there too; recording of sound from devices is not, because the server has none. Not checked on Windows 10 and 11 yet. |
| macOS 10.13+ | Builds, passes the tests; on a machine at GitHub it installs, starts and records the screen with the sound of the computer. Nobody has tried it by hand yet. |
| Ubuntu 24.04+ | Builds, passes the tests; on a machine at GitHub it installs, starts and records the screen. Nobody has tried it by hand yet. On Wayland, assign the shortcut in the keyboard settings: the command is `just-screenshotter`. |

## Install

| System | File | How to install |
| --- | --- | --- |
| Windows 10 and 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Run it. If Windows does not know the program: “More info”, “Run anyway”. |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Drag the program into Applications and open it. When macOS refuses, open System Settings, “Privacy & Security”, and press “Open Anyway”. |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |

GitHub builds these files from the code of this repository, installs each of them and starts the program, and only then publishes them. They have no paid signature, so Windows and macOS warn at the first start.

You can also build the program yourself. You need Node.js 24 or newer, Rust, on Windows also the Microsoft C++ Build Tools, on Ubuntu also `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

The installer appears in `src-tauri/target/release/bundle`.

## Translations

The translations are machine-made and have not been checked by native speakers. If you see a mistake, please correct it in `locales` or in `docs`.

## Contributing and license

How to build, test and change the program: [CONTRIBUTING.md](CONTRIBUTING.md). The license is [MIT](LICENSE).

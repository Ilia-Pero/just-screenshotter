<p align="center"><img src="src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Screenshots and screen recording for Windows, macOS and Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

**English** · [Русский](docs/README.ru.md) · [Español](docs/README.es.md) · [Ελληνικά](docs/README.el.md) · [עברית](docs/README.he.md) · [简体中文](docs/README.zh.md) · [日本語](docs/README.ja.md) · [한국어](docs/README.ko.md) · [Српски](docs/README.sr.md) · [Français](docs/README.fr.md) · [Deutsch](docs/README.de.md) · [Italiano](docs/README.it.md) · [Беларуская](docs/README.be.md) · [Қазақша](docs/README.kk.md)

![The editor over a sample screen: a marker, an oval, an arrow and a label](docs/editor.png)

Press Print Screen, and the program captures every monitor and opens an editor on top of the screen. Select an area, draw on it, then copy, save or upload the result. The same area can be recorded as video.

Between screenshots one process stays in memory with an icon in the tray. On the test machine it held 14–25 MB and used 0 ms of processor time in 10 seconds ([measurements](#measurements)).

## Install

| System | File | How to install |
| --- | --- | --- |
| Windows 10 and 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Run the file. If Windows shows a warning, choose “More info”, then “Run anyway”. |
| macOS 10.13 or later | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Drag the program to Applications and open it. After macOS refuses for the first time, open System Settings, “Privacy & Security”, and press “Open Anyway”. |
| Ubuntu 24.04 or later | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128 or later | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | Open `about:debugging`, “This Firefox”, “Load Temporary Add-on”, and select the file. Firefox removes such an extension when it closes, so load it again after a restart. |
| Chrome, Edge, Opera, Yandex Browser | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | Unpack the archive into a folder you will keep. Open `chrome://extensions`, turn on “Developer mode”, press “Load unpacked” and select the folder. Yandex Browser may turn off an extension that did not come from a store. |

Windows and macOS warn about the files because they are not signed with a paid certificate. GitHub builds them from this repository, installs the programs on clean machines and starts them before a release is published.

## What it does

- Draws with a pencil, a line, an arrow, a frame, an oval, a marker and text, in eight colors and any thickness from 1 to 72 pixels.
- Lets you change a drawing at any time. Click it to move it, pull its ends, recolor it, retype its text or delete it.
- Copies the result, saves it as PNG, or uploads it to [your own server](server/README.md) and copies the link.
- Records the selected area with system audio and the microphone: MP4 on Windows, MOV on macOS, WebM on Linux. Before macOS 13 only the microphone is recorded. “Stop”, the shortcut or the tray icon ends the recording.
- Speaks 14 languages. The settings also hold the folder for files, the shortcuts and starting with the system, which is on by default.

## Keys

| Action | Windows and Linux | macOS |
| --- | --- | --- |
| Screenshot of an area | Print Screen | ⌘⇧9 |
| Screenshot of the whole screen | Shift + Print Screen | not assigned |
| Screenshot of the active window, Windows only | Alt + Print Screen | |
| Video of an area | Ctrl + Shift + Print Screen | ⌘⇧8 |

All of these can be changed in the settings, which open from the tray menu or with `just-screenshotter --settings`. On Wayland shortcuts belong to the desktop: add one in the keyboard settings with the command `just-screenshotter`.

On Windows the same actions can be started with both mouse buttons pressed at once, alone or together with Shift or Ctrl. This is off by default, because listening for the buttons takes about 2 µs of processor time per mouse movement.

In the editor (on macOS, ⌘ instead of Ctrl):

| Key | Action |
| --- | --- |
| Ctrl+C | copies the area |
| Ctrl+S | saves the area to `Pictures/Screenshots` |
| Ctrl+U | uploads the area and copies the link |
| Ctrl+Z | undoes the last change |
| Delete | deletes the selected drawing |
| Mouse wheel | changes the thickness of a line or the size of letters |
| Shift | makes a frame square and an oval round |
| Enter | edits the selected text; while typing, begins a new line |
| Esc | ends typing; otherwise closes the editor |

A double click also opens a text for editing. The spell checker of the system underlines mistakes in Russian and English.

## Browser extension

The extension captures the visible part of a page (Alt+Shift+S) or the whole page (Alt+Shift+P) and opens the same editor in a new tab. One picture holds up to 32,000 pixels of a page in height. The extension works in Firefox and in browsers based on Chromium; it does not record video.

## Privacy

Screenshots and recordings stay on your computer. The program uses the network in two cases: when you upload a screenshot to the server you have set, and when you press “Check for updates”. It has no account or telemetry. The extension looks at a page after you press its button or keys, and makes no network requests.

## Measurements

Windows Server 2022, one monitor of 1920 × 1080, no video card, release build, 2026-11-24.

| What was measured | Result |
| --- | --- |
| Processor time at rest | 0 ms in 10 s |
| Memory at rest | 14 MB before the first screenshot, 25 MB after seven |
| Processes at rest | 1 |
| From the key press to the editor | 0.61–0.75 s in seven runs |
| Size of the program file | 5.2 MB |
| Recording 1920 × 1080, 30 frames per second | 1 processor core, 207 MB |
| Recording 800 × 600, 30 frames per second | 0.4 of a core, 78 MB |

The window of the editor is created for every screenshot and destroyed after it, which is why it takes this long to open. A separate process records video and exits when the recording ends. Recording on a machine with a hardware H.264 encoder has not been measured.

## Tested on

| System | What was checked |
| --- | --- |
| Windows Server 2022 | Screenshots, the editor and video on one monitor. Recording of sound was not checked: the machine has no audio devices. |
| Windows 10 and 11 | Not checked yet. |
| macOS, a GitHub machine | Builds, passes the tests, installs, starts, records the screen with system audio. Not checked by hand. |
| Ubuntu 24.04 and 26.04, GitHub machines | Builds, passes the tests, installs, starts, records the screen. |
| Ubuntu 26.04, a virtual X11 screen | Keys, selection, saving and video work. Wayland was not checked. |
| Firefox 161, Edge 158 | The extension works; checked without a window. Chrome, Opera and Yandex Browser were not checked; they need Chromium 120 or later. |

If you run it on a system that has not been checked, please [write how it went](https://github.com/Ilia-Pero/just-screenshotter/issues).

## Build

You need Node.js 24 or later and Rust; on Windows also the Microsoft C++ Build Tools, on Ubuntu also `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

The installer appears in `src-tauri/target/release/bundle`. Tests, the structure of the code and the rules for changes are described in [CONTRIBUTING.md](CONTRIBUTING.md).

## Translations

English and Russian are the main languages. The other twelve are machine translations that native speakers have not checked. Corrections go to `locales` and `docs`.

## License

[MIT](LICENSE)

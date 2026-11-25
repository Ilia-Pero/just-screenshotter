# Contributing

## Build

You need Node.js 24 or newer and Rust with clippy and rustfmt. On Windows, also the Microsoft C++ Build Tools. On Ubuntu, also `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
npm ci
npm run build
```

`npm run build` runs the linter, the type check, the tests, the bundle, rustfmt, clippy and the tests of Rust. It has to pass without warnings.

| Command | What it does |
| --- | --- |
| `npm run dev` | opens the editor in a browser over a sample screen: <http://127.0.0.1:5391>; `?he` at the end chooses the language; the settings are at <http://127.0.0.1:5391/panel.html> |
| `npm run tauri build` | builds the installer for this system |
| `npm run tauri build -- --debug --no-bundle` | builds the program for a try |
| `cargo run --manifest-path src-tauri/Cargo.toml -p screenshotter-film --example film -- 5 film.mp4 1920 1080 100 100 800 600` | films a part of the screen for five seconds: the size of the monitor, then the area; on macOS the file is `film.mov`, on Linux `film.webm` |
| `python -m unittest discover -s tests -t .` | tests the server, in the folder `server` |

In a debug build, `SCREENSHOTTER_DEBUG_PORT` opens the debugging port of the editor, so a test can drive it without the mouse and the keyboard.

The compiler checks the code for other systems on any machine:

```sh
rustup target add x86_64-unknown-linux-gnu x86_64-apple-darwin
cargo clippy --manifest-path src-tauri/Cargo.toml -p screenshotter-capture -p screenshotter-film --target x86_64-unknown-linux-gnu
cargo clippy --manifest-path src-tauri/Cargo.toml -p screenshotter-capture -p screenshotter-film --target x86_64-apple-darwin
```

## Where things are

| Folder | What is in it |
| --- | --- |
| `src` | the editor, TypeScript without frameworks |
| `src-tauri/src` | the program: the tray icon, the shortcut, the windows of the editor |
| `src-tauri/capture` | taking screenshots: GDI on Windows, `screencapture` on macOS, the portal on Linux |
| `src-tauri/film` | filming in a process of its own: Media Foundation on Windows, ScreenCaptureKit and AVFoundation on macOS, the portal and GStreamer on Linux |
| `src-tauri/mouse` | both buttons of the mouse pressed together, Windows |
| `src-tauri/store` | the settings and the folder with screenshots |
| `src-tauri/upload` | uploading to the server |
| `src-tauri/phrase`, `locales` | texts in 12 languages |
| `server` | the server for screenshots, Python |

## What to keep in mind

**No load at rest.** Between screenshots and films there are no windows, no timers and no polling. A change that breaks this is not accepted; please measure the processor time and the memory at rest and write them into the pull request.

**macOS 10.13.** On High Sierra the system WebKit draws the window, so the editor is built for Safari 11: no Pointer Events, no `gap` in flex, no ResizeObserver, no destructuring. `npm run bundle` fails if the code needs something newer.

**Objects.** They are immutable, there is no `null`, and names are not made with "-er". A test has one assertion, and its name says what the program does. What is left undone is marked in the code with `@todo #issue` next to the place it concerns, as in [puzzle driven development](https://www.yegor256.com/2010/03/04/pdd.html). Such a comment is enough to take the work up. What is planned: [ROADMAP.md](ROADMAP.md).

## Translations

A language is a file in `locales`. To correct one, change its texts and leave the keys and the `{holes}` as they are: tests check that all languages have the same ones. To add one, copy `en.json` and add the language to `src-tauri/phrase/src/language.rs` and `src/languages.ts`. The translated descriptions are `docs/README.*.md`.

## Releases

Set the version in `package.json`, `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`, write the changes and the date into `CHANGELOG.md`, and push a tag such as `0.2.0`. GitHub then builds the installers, installs each of them, starts the program, and only after that publishes the release.

## Licenses of dependencies

The code is under MIT. Dependencies are under MIT, Apache 2.0, BSD, ISC, Zlib, Unicode, BSL, MPL 2.0 and CDLA Permissive 2.0. The full list: `cargo tree --manifest-path src-tauri/Cargo.toml --format "{p} {l}"`. The installers carry the texts: `npm run licenses` collects them into `src-tauri/licenses.txt`.

# Contributing

## Build and test

You need Node.js 24 or later and Rust with clippy and rustfmt; on Windows also the Microsoft C++ Build Tools, on Ubuntu also `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
npm ci
npm run build
```

`npm run build` runs the linter, the type check, the tests, the bundler, rustfmt, clippy and the Rust tests. A pull request has to pass it without warnings.

| Command | What it does |
| --- | --- |
| `npm run dev` | opens the editor in a browser over a sample screen at <http://127.0.0.1:5391> (`?he` at the end selects a language); the settings are at <http://127.0.0.1:5391/panel.html> |
| `npm run extension` | builds the browser extension into `extension/packed/firefox` and `extension/packed/chrome` |
| `npm run tauri build` | builds the installer for your system |
| `npm run tauri build -- --debug --no-bundle` | builds a debug version without an installer |
| `cargo run --manifest-path src-tauri/Cargo.toml -p screenshotter-film --example film -- 5 film.mp4 1920 1080 100 100 800 600` | records a part of the screen for five seconds. The numbers are the size of the monitor and the area. Use `film.mov` on macOS and `film.webm` on Linux |
| `python -m unittest discover -s tests -t .` | tests the server; run it in the folder `server` |

In a debug build the variable `SCREENSHOTTER_DEBUG_PORT` opens a debugging port of the editor, so a test can control it without the mouse and the keyboard.

Code for other systems can be checked on any machine:

```sh
rustup target add x86_64-unknown-linux-gnu x86_64-apple-darwin
cargo clippy --manifest-path src-tauri/Cargo.toml -p screenshotter-capture -p screenshotter-film --target x86_64-unknown-linux-gnu
cargo clippy --manifest-path src-tauri/Cargo.toml -p screenshotter-capture -p screenshotter-film --target x86_64-apple-darwin
```

## Structure

| Folder | Contents |
| --- | --- |
| `src` | the editor and the settings, TypeScript without frameworks |
| `extension` | the browser extension, which uses the same editor |
| `src-tauri/src` | the program: the tray icon, shortcuts, windows |
| `src-tauri/capture` | screenshots: GDI on Windows, `screencapture` on macOS, the portal on Linux |
| `src-tauri/film` | video, recorded in a separate process: Media Foundation on Windows, ScreenCaptureKit and AVFoundation on macOS, the portal and GStreamer on Linux |
| `src-tauri/keys` | shortcuts on X11 |
| `src-tauri/mouse` | both mouse buttons pressed at once, Windows |
| `src-tauri/release` | the version of the latest release |
| `src-tauri/spelling` | the spell checker of the system |
| `src-tauri/store` | the settings and the folder with screenshots |
| `src-tauri/upload` | uploading to the server |
| `src-tauri/phrase`, `locales` | texts in 14 languages |
| `server` | the server for screenshots, Python |

## Rules

**Idle load.** Between screenshots and recordings the program has no windows, timers or polling. If your change affects this, measure processor time and memory at rest and put the numbers into the pull request.

**macOS 10.13.** High Sierra draws the windows with its own WebKit, so the code is built for Safari 11, which lacks Pointer Events, `gap` in flex, ResizeObserver and destructuring. `npm run bundle` fails if the code needs a newer browser.

**Objects.** The code follows [Elegant Objects](https://www.elegantobjects.org/): objects are immutable, `null` is not used, names do not end in "-er". A test has one assertion, and its name describes behavior.

**Unfinished work** is marked with `@todo #issue` next to the code it concerns, as in [puzzle driven development](https://www.yegor256.com/2010/03/04/pdd.html). Plans are in [ROADMAP.md](ROADMAP.md).

## Translations

Each language is a file in `locales`. To correct one, change the texts and keep the keys and the `{placeholders}`; tests compare them with the other languages. To add one, copy `en.json` and list the language in `src-tauri/phrase/src/language.rs` and `src/languages.ts`. Translated descriptions are in `docs/README.*.md`.

## Releases

1. Set the version in `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json` and `extension/web/manifest.json`.
2. Describe the changes in `CHANGELOG.md` under the version and the date.
3. Push a tag such as `0.2.0`.

GitHub builds the installers, installs each one, starts the program and then publishes the release. If the repository has the secrets `AMO_KEY` and `AMO_SECRET`, the release also sends the extension to Firefox Add-ons with the texts from `extension/store/firefox.json`. The workflow `catalog` does the same for a release that has already been published.

## Licenses of dependencies

The code is under MIT. Dependencies are under MIT, Apache 2.0, BSD, ISC, Zlib, Unicode, BSL, MPL 2.0 and CDLA Permissive 2.0; `cargo tree --manifest-path src-tauri/Cargo.toml --format "{p} {l}"` prints the full list. `npm run licenses` collects the texts into `src-tauri/licenses.txt`, which goes into the installers.

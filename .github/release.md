
## Files

- `just-screenshotter-windows.exe` for Windows 10 and 11. If Windows shows a warning, choose “More info”, then “Run anyway”.
- `just-screenshotter-macos.dmg` for macOS 10.13 and later, Intel and Apple Silicon. Drag the program to Applications and open it. After macOS refuses for the first time, open System Settings, “Privacy & Security”, and press “Open Anyway”.
- `just-screenshotter-ubuntu.deb` for Ubuntu 24.04 and later, x86-64: `sudo apt install ./just-screenshotter-ubuntu.deb`
- `just-screenshotter-firefox.zip`, the extension for Firefox 128 and later. Open `about:debugging`, “This Firefox”, “Load Temporary Add-on”, and select the file. Firefox removes such an extension when it closes.
- `just-screenshotter-chrome.zip`, the extension for Chrome, Edge, Opera and Yandex Browser (Chromium 120 and later). Unpack it, open `chrome://extensions`, turn on “Developer mode”, press “Load unpacked” and select the folder. Yandex Browser may turn off an extension that did not come from a store.

GitHub built these files from the tagged code, installed the programs and started them. They are not signed with a paid certificate, so Windows and macOS warn at the first start. The program has not been checked by hand on Windows 10 and 11, on a Mac or on Ubuntu; [reports](https://github.com/Ilia-Pero/just-screenshotter/issues) are welcome.

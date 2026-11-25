<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Screenshots und Bildschirmaufnahmen für Windows, macOS und Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · **Deutsch** · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![Der Editor über einem Beispielbildschirm: Marker, Oval, Pfeil und Text](editor.png)

Nach einem Druck auf Print Screen nimmt das Programm alle Bildschirme auf und öffnet darüber einen Editor. Dort wählen Sie einen Bereich, zeichnen darauf und kopieren, speichern oder senden das Bild. Derselbe Bereich lässt sich als Video aufnehmen. Zwischen den Aufnahmen bleibt ein Prozess mit einem Symbol im Infobereich im Speicher.

## Installation

| System | Datei | So wird installiert |
| --- | --- | --- |
| Windows 10, 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Datei starten. Wenn Windows warnt: „Weitere Informationen“, dann „Trotzdem ausführen“. |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Programm in „Programme“ ziehen und öffnen. Nach der ersten Ablehnung in den Systemeinstellungen „Datenschutz & Sicherheit“ öffnen und „Dennoch öffnen“ wählen. |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | `about:debugging` öffnen, „Dieser Firefox“, „Temporäres Add-on laden“ und die Datei wählen. Nach einem Neustart von Firefox erneut laden. |
| Chrome, Edge, Opera, Yandex Browser | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | Archiv in einen Ordner entpacken, der bestehen bleibt. `chrome://extensions` öffnen, „Entwicklermodus“ einschalten, „Entpackte Erweiterung laden“ und den Ordner wählen. |

Windows und macOS warnen, weil die Dateien keine bezahlte Signatur haben. GitHub baut sie aus diesem Repository, installiert die Programme auf sauberen Rechnern und startet sie, bevor eine Version veröffentlicht wird.

## Funktionen

- Zeichnen mit Stift, Linie, Pfeil, Rahmen, Oval, Marker und Text: acht Farben, Stärke von 1 bis 72 Pixel.
- Gezeichnetes lässt sich jederzeit verschieben, umfärben, in der Größe ändern, neu tippen oder löschen.
- Das Bild wird kopiert, als PNG gespeichert oder auf [Ihren eigenen Server](../server/README.md) geladen.
- Video eines Bereichs mit Ton des Computers und Mikrofon: MP4 unter Windows, MOV unter macOS, WebM unter Linux. Vor macOS 13 wird nur das Mikrofon aufgenommen.
- 14 Sprachen; Tasten und Ordner werden in den Einstellungen festgelegt.

## Tasten

| Aktion | Windows, Linux | macOS |
| --- | --- | --- |
| Screenshot eines Bereichs | Print Screen | ⌘⇧9 |
| Screenshot des ganzen Bildschirms | Shift + Print Screen | |
| Screenshot des aktiven Fensters, nur Windows | Alt + Print Screen | |
| Video eines Bereichs | Ctrl + Shift + Print Screen | ⌘⇧8 |

Die Tasten lassen sich in den Einstellungen ändern. Im Editor kopiert Strg+C, Strg+S speichert, Strg+Z macht rückgängig, Esc schließt.

## Browser-Erweiterung

Nimmt den sichtbaren Teil einer Seite (Alt+Shift+S) oder die ganze Seite (Alt+Shift+P) auf und öffnet denselben Editor in einem neuen Tab. Läuft in Firefox und in Browsern auf Chromium-Basis.

## Daten und Netzwerk

Bilder und Aufnahmen bleiben auf Ihrem Computer. Das Programm nutzt das Netz, wenn Sie ein Bild auf Ihren Server laden und wenn Sie auf „Nach Updates suchen“ klicken. Ein Konto oder Telemetrie gibt es nicht.

## Mehr

Messwerte, geprüfte Systeme und der Bau des Programms sind [auf Englisch](../README.md) beschrieben.

Dieser Text wurde maschinell übersetzt und nicht von Muttersprachlern geprüft. Korrekturen gehören in den Ordner `docs`.

[MIT](../LICENSE)

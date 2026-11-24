<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Ein Screenshot-Werkzeug ohne Last im Ruhezustand für Windows, macOS und Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · **Deutsch** · [Italiano](README.it.md)

![Der Editor über einem Beispielbildschirm: Marker, Rahmen, Pfeil und Text](editor.png)

Drücken Sie Print Screen, wählen Sie einen Bereich, zeichnen Sie darauf und kopieren, speichern oder senden Sie das Bild. Zwischen den Aufnahmen wartet das Programm nur auf die Taste: Es hat keine Fenster, keine Timer und belegt den Prozessor nicht.

## Was es kann

- Nimmt alle Bildschirme auf und öffnet den Editor darüber.
- Zeichnet mit Stift, Linie, Pfeil, Rahmen, Marker und Text in acht Farben.
- Kopiert das Bild, speichert es als PNG oder lädt es auf [Ihren eigenen Server](../server/README.md) und kopiert den Link.
- Spricht 12 Sprachen. Zuerst die Sprache des Systems; eine andere wird im Menü gewählt.
- Sendet von sich aus nichts: kein Konto, keine Telemetrie, keine Suche nach Updates. Das Netz wird nur genutzt, um ein Bild auf den von Ihnen angegebenen Server zu laden.

## Tasten im Editor

| Taste | Was sie tut |
| --- | --- |
| Ctrl+C | kopiert den Bereich |
| Ctrl+S | speichert den Bereich in `Bilder/Screenshots` |
| Ctrl+U | lädt den Bereich hoch, der Link kommt in die Zwischenablage |
| Ctrl+Z | macht die letzte Zeichnung rückgängig |
| Esc | schließt den Editor |

Unter macOS ⌘ statt Strg.

Im Menü des Symbols in der Taskleiste stehen die Tastenkombination, der Ordner, die Sprache, die Serveradresse und der Start mit dem System. Auch ein zweiter Start des Programms macht einen Screenshot.

## Wie leicht es ist

Gemessen am 24.11.2026 unter Windows Server 2022, Bildschirm 1920 × 1080, optimierter Build:

| Was | Ergebnis |
| --- | --- |
| Prozessor im Ruhezustand | 0 ms in 10 s |
| Speicher im Ruhezustand | 14 MB vor der ersten Aufnahme, 25 MB nach sieben |
| Prozesse im Ruhezustand | 1 |
| Vom Tastendruck bis zum Editor | 0,61–0,75 s in sieben Läufen |
| Größe des Programms | 5,0 MB |

Der Editor braucht diese Zeit, weil sein Fenster für jede Aufnahme erstellt und danach zerstört wird. Das ist der Preis für null Last im Ruhezustand.

## Stand

| System | Stand |
| --- | --- |
| Windows | Funktioniert unter Windows Server 2022 mit einem Bildschirm. Unter Windows 10 und 11 noch nicht geprüft. |
| macOS 10.13+ | Lässt sich bauen und besteht die Tests. Noch niemand hat es gestartet. |
| Ubuntu 24.04+ | Lässt sich bauen und besteht die Tests. Noch niemand hat es gestartet. Unter Wayland legen Sie die Tastenkombination in den Tastatureinstellungen fest: Der Befehl ist `just-screenshotter`. |

## Installation

Fertige Dateien gibt es noch nicht, das Programm muss gebaut werden. Nötig sind Node.js 24 oder neuer und Rust, unter Windows auch die Microsoft C++ Build Tools, unter Ubuntu auch `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

Das Installationsprogramm erscheint in `src-tauri/target/release/bundle`. Es ist nicht signiert, deshalb fragen Windows und macOS, ob Sie ihm vertrauen.

## Übersetzungen

Die Übersetzungen sind maschinell und wurden nicht von Muttersprachlern geprüft. Wenn Sie einen Fehler sehen, berichtigen Sie ihn bitte in `locales` oder in `docs`.

## Mitmachen und Lizenz

So wird das Programm gebaut, geprüft und geändert: [CONTRIBUTING.md](../CONTRIBUTING.md). Die Lizenz ist [MIT](../LICENSE).

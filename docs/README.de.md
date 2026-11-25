<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Ein Screenshot-Werkzeug ohne Last im Ruhezustand für Windows, macOS und Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · **Deutsch** · [Italiano](README.it.md)

![Der Editor über einem Beispielbildschirm: Marker, Rahmen, Pfeil und Text](editor.png)

Drücken Sie Print Screen, wählen Sie einen Bereich, zeichnen Sie darauf und kopieren, speichern oder senden Sie das Bild oder nehmen Sie den Bereich als Video auf. Zwischen den Aufnahmen wartet das Programm nur auf die Taste: Es hat keine Fenster, keine Timer und belegt den Prozessor nicht.

## Was es kann

- Nimmt alle Bildschirme auf und öffnet den Editor darüber.
- Zeichnet mit Stift, Linie, Pfeil, Rahmen, Marker und Text in acht Farben.
- Ändert bereits Gezeichnetes: Ein Klick wählt eine Markierung, dann ziehen Sie sie oder ihre Enden, ändern Stärke und Farbe, tippen den Text neu oder löschen sie.
- Kopiert das Bild, speichert es als PNG oder lädt es auf [Ihren eigenen Server](../server/README.md) und kopiert den Link.
- Nimmt den Bereich als Video mit Ton auf: MP4 unter Windows, MOV unter macOS, WebM unter Linux. Sie wählen, ob der Ton des Computers und das Mikrofon aufgenommen werden; unter macOS vor Version 13 nur das Mikrofon. Die Schaltfläche „Beenden“, die Tastenkombination oder das Symbol beendet die Aufnahme.
- Spricht 12 Sprachen. Die Sprache wird im Editor, in den Einstellungen und im Installationsprogramm für Windows gewählt; bis dahin gilt die Sprache des Systems.
- Sendet von sich aus nichts: kein Konto, keine Telemetrie, keine Suche nach Updates. Das Netz wird nur genutzt, um ein Bild auf den von Ihnen angegebenen Server zu laden.

## Tasten und Maus

| Was sie tun | Windows und Linux | macOS |
| --- | --- | --- |
| Screenshot eines Bereichs | Print Screen | ⌘⇧9 |
| Screenshot des ganzen Bildschirms | Shift + Print Screen | — |
| Screenshot des aktiven Fensters (Windows) | Alt + Print Screen | — |
| Video eines Bereichs | Ctrl + Shift + Print Screen | ⌘⇧8 |

Andere Tasten wählen Sie in den Einstellungen. Unter Windows starten dasselbe auch die Maustasten: beide zugleich, allein, mit Umschalt oder mit Strg. Anfangs ist das aus, denn solange das Programm die Maustasten beobachtet, kostet jede Mausbewegung etwa 2 µs Prozessorzeit; in Ruhe nichts. Unter Wayland legt der Desktop die Taste fest.

## Tasten im Editor

| Taste | Was sie tut |
| --- | --- |
| Ctrl+C | kopiert den Bereich |
| Ctrl+S | speichert den Bereich in `Bilder/Screenshots` |
| Ctrl+U | lädt den Bereich hoch, der Link kommt in die Zwischenablage |
| Ctrl+Z | macht die letzte Zeichnung rückgängig |
| Delete | löscht die gewählte Markierung |
| Esc | schließt den Editor |

Unter macOS ⌘ statt Strg.

Markierungen werden gewählt, wenn kein Werkzeug aktiv ist. Ein Doppelklick oder Enter ändert einen Text. Das Mausrad ändert die Stärke.

Auch der Rest steht in den Einstellungen: was mit dem Video aufgenommen wird, der Ordner, die Sprache, das Hochladen und der Start mit dem System, der an ist, bis Sie ihn ausschalten. Die Einstellungen öffnet das Menü des Symbols oder der Start des Programms mit `--settings`. Auch ein zweiter Start ohne das macht einen Screenshot.

## Wie leicht es ist

Gemessen am 24.11.2026 unter Windows Server 2022, Bildschirm 1920 × 1080, optimierter Build:

| Was | Ergebnis |
| --- | --- |
| Prozessor im Ruhezustand | 0 ms in 10 s |
| Speicher im Ruhezustand | 14 MB vor der ersten Aufnahme, 25 MB nach sieben |
| Prozesse im Ruhezustand | 1 |
| Vom Tastendruck bis zum Editor | 0,61–0,75 s in sieben Läufen |
| Größe des Programms | 5,2 MB |

Der Editor braucht diese Zeit, weil sein Fenster für jede Aufnahme erstellt und danach zerstört wird. Das ist der Preis für null Last im Ruhezustand.

Das Video nimmt ein eigener Prozess auf, der am Ende samt seinem Speicher verschwindet. Auf diesem Server ohne Grafikkarte belegen 30 Bilder pro Sekunde bei 1920 × 1080 einen Prozessorkern und 207 MB, bei 800 × 600 0,4 Kerne und 78 MB. Eine Grafikkarte, die H.264 kodiert, übernimmt diese Arbeit; das ist noch nicht gemessen.

## Stand

| System | Stand |
| --- | --- |
| Windows | Funktioniert unter Windows Server 2022 mit einem Bildschirm. Auch Video wurde dort geprüft; die Tonaufnahme von Geräten nicht, weil der Server keine hat. Unter Windows 10 und 11 noch nicht geprüft. |
| macOS 10.13+ | Lässt sich bauen und besteht die Tests; auf einer Maschine bei GitHub lässt es sich installieren, startet und nimmt den Bildschirm mit dem Ton des Computers auf. Von Hand hat es noch niemand ausprobiert. |
| Ubuntu 24.04+ | Lässt sich bauen und besteht die Tests; auf einer Maschine bei GitHub lässt es sich installieren, startet und nimmt den Bildschirm auf. Von Hand hat es noch niemand ausprobiert. Unter Wayland legen Sie die Tastenkombination in den Tastatureinstellungen fest: Der Befehl ist `just-screenshotter`. |

## Installation

| System | Datei | So wird installiert |
| --- | --- | --- |
| Windows 10 und 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Starten Sie sie. Wenn Windows das Programm nicht kennt: „Weitere Informationen“, „Trotzdem ausführen“. |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Ziehen Sie das Programm in „Programme“ und öffnen Sie es. Wenn macOS es ablehnt, öffnen Sie die Systemeinstellungen, „Datenschutz und Sicherheit“, und klicken Sie auf „Dennoch öffnen“. |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |

GitHub baut diese Dateien aus dem Code dieses Repositorys, installiert jede, startet das Programm und veröffentlicht sie erst dann. Sie haben keine bezahlte Signatur, deshalb warnen Windows und macOS beim ersten Start.

Sie können das Programm auch selbst bauen. Nötig sind Node.js 24 oder neuer und Rust, unter Windows auch die Microsoft C++ Build Tools, unter Ubuntu auch `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

Das Installationsprogramm erscheint in `src-tauri/target/release/bundle`.

## Übersetzungen

Die Übersetzungen sind maschinell und wurden nicht von Muttersprachlern geprüft. Wenn Sie einen Fehler sehen, berichtigen Sie ihn bitte in `locales` oder in `docs`.

## Mitmachen und Lizenz

So wird das Programm gebaut, geprüft und geändert: [CONTRIBUTING.md](../CONTRIBUTING.md). Die Lizenz ist [MIT](../LICENSE).

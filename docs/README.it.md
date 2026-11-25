<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Schermate e registrazione dello schermo per Windows, macOS e Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · **Italiano** · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![L’editor sopra una schermata di esempio: evidenziatore, ovale, freccia e testo](editor.png)

Con il tasto Print Screen il programma cattura tutti i monitor e apre un editor sopra lo schermo. Si seleziona un’area, ci si disegna e poi si copia, si salva o si invia l’immagine. La stessa area può essere registrata in video. Tra una schermata e l’altra resta in memoria un processo con un’icona nell’area di notifica.

## Installazione

| Sistema | File | Come installare |
| --- | --- | --- |
| Windows 10, 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Avvia il file. Se Windows mostra un avviso, scegli “Ulteriori informazioni” e poi “Esegui comunque”. |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Trascina il programma in Applicazioni e aprilo. Dopo il primo rifiuto di macOS apri Impostazioni di Sistema, “Privacy e sicurezza”, e premi “Apri comunque”. |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | Apri `about:debugging`, “Questo Firefox”, “Carica componente aggiuntivo temporaneo” e scegli il file. Dopo un riavvio di Firefox caricalo di nuovo. |
| Chrome, Edge, Opera, Yandex Browser | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | Estrai l’archivio in una cartella da conservare. Apri `chrome://extensions`, attiva la modalità sviluppatore, premi “Carica estensione non pacchettizzata” e scegli la cartella. |

Windows e macOS avvisano perché i file non hanno una firma a pagamento. GitHub li compila da questo repository, installa i programmi su macchine pulite e li avvia prima di pubblicare una versione.

## Funzioni

- Disegno con matita, linea, freccia, cornice, ovale, evidenziatore e testo: otto colori, spessore da 1 a 72 pixel.
- Un disegno può essere spostato, ricolorato, ridimensionato, riscritto o eliminato in qualsiasi momento.
- L’immagine viene copiata, salvata in PNG o inviata al [tuo server](../server/README.md).
- Video di un’area con l’audio del computer e il microfono: MP4 su Windows, MOV su macOS, WebM su Linux. Prima di macOS 13 si registra solo il microfono.
- 14 lingue; i tasti e la cartella si scelgono nelle impostazioni.

## Tasti

| Azione | Windows, Linux | macOS |
| --- | --- | --- |
| Schermata di un’area | Print Screen | ⌘⇧9 |
| Schermata intera | Shift + Print Screen | |
| Schermata della finestra attiva, solo Windows | Alt + Print Screen | |
| Video di un’area | Ctrl + Shift + Print Screen | ⌘⇧8 |

I tasti si cambiano nelle impostazioni. Nell’editor Ctrl+C copia, Ctrl+S salva, Ctrl+Z annulla, Esc chiude.

## Estensione per browser

Cattura la parte visibile di una pagina (Alt+Shift+S) o la pagina intera (Alt+Shift+P) e apre lo stesso editor in una nuova scheda. Funziona in Firefox e nei browser basati su Chromium.

## Dati e rete

Schermate e registrazioni restano sul tuo computer. Il programma usa la rete quando invii una schermata al tuo server e quando premi “Cerca aggiornamenti”. Non ci sono account né telemetria.

## Altre informazioni

Le misure, i sistemi verificati e la compilazione sono descritti [in inglese](../README.md).

Questo testo è una traduzione automatica non controllata da madrelingua. Le correzioni vanno nella cartella `docs`.

[MIT](../LICENSE)

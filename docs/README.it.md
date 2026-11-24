<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Uno strumento per screenshot senza carico a riposo per Windows, macOS e Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · **Italiano**

![L’editor sopra uno schermo di esempio: un evidenziatore, una cornice, una freccia e un testo](editor.png)

Premi Print Screen, seleziona un’area, disegnaci sopra e poi copia, salva o carica l’immagine. Tra uno screenshot e l’altro il programma aspetta solo il tasto: non ha finestre né timer e non occupa il processore.

## Che cosa fa

- Cattura tutti i monitor e apre l’editor sopra di essi.
- Disegna con matita, linea, freccia, cornice, evidenziatore e testo in otto colori.
- Copia l’immagine, la salva in PNG o la carica sul [tuo server](../server/README.md) e copia il link.
- Parla 12 lingue. All’inizio usa quella del sistema; un’altra si sceglie nel menu.
- Non invia nulla da solo: nessun account, nessuna telemetria, nessun controllo degli aggiornamenti. Usa la rete solo per caricare un’immagine sul server che hai indicato.

## Tasti nell’editor

| Tasto | Che cosa fa |
| --- | --- |
| Ctrl+C | copia l’area |
| Ctrl+S | salva l’area in `Immagini/Screenshots` |
| Ctrl+U | carica l’area, il link va negli appunti |
| Ctrl+Z | annulla l’ultimo disegno |
| Esc | chiude l’editor |

Su macOS, ⌘ al posto di Ctrl.

Nel menu dell’icona ci sono la scorciatoia, la cartella, la lingua, l’indirizzo del server e l’avvio con il sistema. Anche avviare il programma una seconda volta cattura lo schermo.

## Quanto è leggero

Misurato il 24.11.2026 su Windows Server 2022, schermo 1920 × 1080, build ottimizzata:

| Misura | Risultato |
| --- | --- |
| Processore a riposo | 0 ms in 10 s |
| Memoria a riposo | 14 MB prima del primo screenshot, 25 MB dopo sette |
| Processi a riposo | 1 |
| Dal tasto all’editor | 0,61–0,75 s in sette prove |
| Dimensione del programma | 5,0 MB |

L’editor impiega questo tempo perché la sua finestra viene creata per ogni screenshot e poi distrutta. È il prezzo del carico nullo a riposo.

## Stato

| Sistema | Stato |
| --- | --- |
| Windows | Funziona su Windows Server 2022 con un monitor. Non ancora verificato su Windows 10 e 11. |
| macOS 10.13+ | Si compila e supera i test. Nessuno l’ha ancora avviato. |
| Ubuntu 24.04+ | Si compila e supera i test. Nessuno l’ha ancora avviato. Su Wayland assegna la scorciatoia nelle impostazioni della tastiera: il comando è `just-screenshotter`. |

## Installazione

Non ci sono ancora file pronti da scaricare, quindi il programma va compilato. Servono Node.js 24 o successivo e Rust, su Windows anche Microsoft C++ Build Tools, su Ubuntu anche `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

L’installer compare in `src-tauri/target/release/bundle`. Non è firmato, quindi Windows e macOS chiederanno se ti fidi.

## Traduzioni

Le traduzioni sono automatiche e non sono state controllate da madrelingua. Se vedi un errore, correggilo in `locales` o in `docs`.

## Partecipare e licenza

Come compilare, provare e modificare il programma: [CONTRIBUTING.md](../CONTRIBUTING.md). La licenza è [MIT](../LICENSE).

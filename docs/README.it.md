<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Uno strumento per screenshot senza carico a riposo per Windows, macOS e Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · **Italiano** · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![L’editor sopra uno schermo di esempio: un evidenziatore, un ovale, una freccia e un testo](editor.png)

Premi Print Screen, seleziona un’area, disegnaci sopra e poi copia, salva o carica l’immagine, oppure registra l’area come video. Tra uno screenshot e l’altro il programma aspetta solo il tasto: non ha finestre né timer e non occupa il processore.

## Che cosa fa

- Cattura tutti i monitor e apre l’editor sopra di essi.
- Disegna con matita, linea, freccia, cornice, ovale, evidenziatore e testo in otto colori.
- Modifica ciò che è già disegnato: un clic sceglie un segno, poi lo si trascina o se ne spostano le estremità, si cambiano spessore e colore, si riscrive il testo o lo si elimina.
- Copia l’immagine, la salva in PNG o la carica sul [tuo server](../server/README.md) e copia il link.
- Registra l’area come video con audio: MP4 su Windows, MOV su macOS, WebM su Linux. Scegli tu se registrare l’audio del computer e il microfono; su macOS precedente a 13 solo il microfono. La registrazione termina con il pulsante Ferma, la scorciatoia o l’icona.
- Parla 14 lingue. La lingua si sceglie nelle impostazioni e nel programma di installazione per Windows; fino ad allora è quella del sistema.
- Non invia nulla da solo: nessun account, nessuna telemetria. Usa la rete solo per caricare un’immagine sul server che hai indicato e per cercare una nuova versione quando premi il pulsante nelle impostazioni.

## Tasti e mouse

| Che cosa fanno | Windows e Linux | macOS |
| --- | --- | --- |
| Screenshot di un’area | Print Screen | ⌘⇧9 |
| Screenshot di tutto lo schermo | Shift + Print Screen | — |
| Screenshot della finestra attiva (Windows) | Alt + Print Screen | — |
| Video di un’area | Ctrl + Shift + Print Screen | ⌘⇧8 |

Altri tasti si scelgono nelle impostazioni. Su Windows le stesse cose si avviano con i pulsanti del mouse: entrambi insieme, da soli, con Maiusc o con Ctrl. All’inizio è disattivato, perché finché il programma osserva i pulsanti ogni movimento del mouse richiede circa 2 µs di processore; a riposo niente. Su Wayland il tasto si assegna nelle impostazioni del sistema.

## Tasti nell’editor

| Tasto | Che cosa fa |
| --- | --- |
| Ctrl+C | copia l’area |
| Ctrl+S | salva l’area in `Immagini/Screenshots` |
| Ctrl+U | carica l’area, il link va negli appunti |
| Ctrl+Z | annulla l’ultimo disegno |
| Delete | elimina il segno scelto |
| Esc | chiude l’editor |

Su macOS, ⌘ al posto di Ctrl.

Un clic sceglie un segno con qualsiasi strumento. Il doppio clic o Invio cambia un testo. La rotella del mouse cambia lo spessore.

In un testo, Invio inizia una nuova riga; un clic altrove o Esc termina il testo. La rotella cambia la dimensione delle lettere. Il sistema sottolinea gli errori in russo e in inglese, e il tasto destro del mouse propone correzioni. Con Maiusc la cornice diventa un quadrato e l’ovale un cerchio.

Anche il resto è nelle impostazioni: che cosa registrare con il video, la cartella, la lingua, il caricamento e l’avvio con il sistema, attivo finché non lo disattivi. Le impostazioni si aprono dal menu dell’icona o avviando il programma con `--settings`. Anche avviarlo una seconda volta senza di esso cattura lo schermo.

## L’estensione per il browser

Per Firefox e per i browser basati su Chromium: Chrome, Yandex Browser, Edge, Opera.

Cattura la parte visibile di una pagina o la pagina intera e apre lo stesso editor in una nuova scheda: selezionare, disegnare, copiare, salvare. I tasti sono Alt+Maiusc+S e Alt+Maiusc+P. Guarda una pagina solo quando lo chiedi, non invia nulla e ricorda solo la lingua. Non registra video, e un’immagine è alta al massimo 32.000 pixel.

## Quanto è leggero

Misurato il 24.11.2026 su Windows Server 2022, schermo 1920 × 1080, build ottimizzata:

| Misura | Risultato |
| --- | --- |
| Processore a riposo | 0 ms in 10 s |
| Memoria a riposo | 14 MB prima del primo screenshot, 25 MB dopo sette |
| Processi a riposo | 1 |
| Dal tasto all’editor | 0,61–0,75 s in sette prove |
| Dimensione del programma | 5,2 MB |

L’editor impiega questo tempo perché la sua finestra viene creata per ogni screenshot e poi distrutta. È il prezzo del carico nullo a riposo.

Il video è registrato da un processo a parte, che alla fine scompare con tutta la sua memoria. Su questo server, senza scheda video, 30 fotogrammi al secondo a 1920 × 1080 occupano un core del processore e 207 MB; a 800 × 600, 0,4 core e 78 MB. Una scheda video che codifica H.264 si fa carico di questo lavoro; non è ancora stato misurato.

## Stato

| Sistema | Stato |
| --- | --- |
| Windows | Funziona su Windows Server 2022 con un monitor. Anche il video è stato verificato lì; la registrazione dell’audio dai dispositivi no, perché il server non ne ha. Non ancora verificato su Windows 10 e 11. |
| macOS 10.13+ | Si compila e supera i test; su una macchina di GitHub si installa, si avvia e registra lo schermo con l’audio del computer. Nessuno l’ha ancora provato a mano. |
| Ubuntu 24.04+ | Si compila e supera i test; su una macchina di GitHub si installa, si avvia e registra lo schermo. Su uno schermo X11 virtuale di Ubuntu 26.04 funzionano i tasti, la selezione, il salvataggio e il video; su Wayland non è stato verificato. Nessuno l’ha ancora provato a mano. Su Wayland assegna la scorciatoia nelle impostazioni della tastiera: il comando è `just-screenshotter`. |
| Firefox 128+ | Funziona in Firefox 161, verificata senza finestra. Non ancora firmata da Mozilla, quindi va ricaricata dopo il riavvio di Firefox. |
| Chrome, Yandex Browser, Edge, Opera | Funziona in Edge 158, verificata senza finestra. Gli altri non sono stati verificati; serve Chromium 120 o successivo. |

## Installazione

| Sistema | File | Come si installa |
| --- | --- | --- |
| Windows 10 e 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Avvialo. Se Windows non conosce il programma: «Ulteriori informazioni», «Esegui comunque». |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Trascina il programma in Applicazioni e aprilo. Quando macOS rifiuta, vai in Impostazioni di Sistema, «Privacy e sicurezza», e fai clic su «Apri comunque». |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | Apri `about:debugging`, «Questo Firefox», «Carica componente aggiuntivo temporaneo», e scegli il file. Dopo il riavvio di Firefox ricaricalo. |
| Chrome, Yandex Browser, Edge, Opera | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | Decomprimilo, apri `chrome://extensions`, attiva la «Modalità sviluppatore», premi «Carica estensione non pacchettizzata» e scegli la cartella; non cancellarla. Yandex Browser si fida solo dei cataloghi e può disattivare l’estensione. |

GitHub compila questi file dal codice di questo repository, installa ciascuno, avvia il programma e solo dopo li pubblica. Non hanno una firma a pagamento, quindi Windows e macOS avvisano al primo avvio.

Puoi anche compilare il programma da te. Servono Node.js 24 o successivo e Rust, su Windows anche Microsoft C++ Build Tools, su Ubuntu anche `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

L’installer compare in `src-tauri/target/release/bundle`.

## Traduzioni

Le traduzioni sono automatiche e non sono state controllate da madrelingua. Se vedi un errore, correggilo in `locales` o in `docs`.

## Partecipare e licenza

Come compilare, provare e modificare il programma: [CONTRIBUTING.md](../CONTRIBUTING.md). La licenza è [MIT](../LICENSE).

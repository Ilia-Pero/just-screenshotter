<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Una herramienta de capturas sin carga en reposo para Windows, macOS y Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · **Español** · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![El editor sobre una pantalla de muestra: un marcador, un marco, una flecha y un texto](editor.png)

Pulse Print Screen, seleccione un área, dibuje sobre ella y copie, guarde o suba la imagen, o grabe el área en vídeo. Entre capturas el programa solo espera la tecla: no tiene ventanas ni temporizadores y no ocupa el procesador.

## Qué hace

- Captura todos los monitores y abre el editor encima de ellos.
- Dibuja con lápiz, línea, flecha, marco, marcador y texto en ocho colores.
- Cambia lo ya dibujado: un clic elige una marca y después puede arrastrarla o mover sus extremos, cambiar el grosor y el color, reescribir el texto o borrarla.
- Copia la imagen, la guarda en PNG o la sube a [su propio servidor](../server/README.md) y copia el enlace.
- Graba el área en vídeo con sonido: MP4 en Windows, MOV en macOS, WebM en Linux. Usted elige si grabar el sonido del ordenador y el micrófono; en macOS anterior a 13, solo el micrófono. La grabación termina con el botón Detener, el atajo o el icono de la bandeja.
- Habla 14 idiomas. El idioma se elige en los ajustes y en el instalador para Windows; hasta entonces es el del sistema.
- No envía nada por su cuenta: no hay cuenta, telemetría ni búsqueda de actualizaciones. Solo usa la red para subir una imagen al servidor que usted indique.

## Teclas y ratón

| Qué hacen | Windows y Linux | macOS |
| --- | --- | --- |
| Captura de un área | Print Screen | ⌘⇧9 |
| Captura de toda la pantalla | Shift + Print Screen | — |
| Captura de la ventana activa (Windows) | Alt + Print Screen | — |
| Vídeo de un área | Ctrl + Shift + Print Screen | ⌘⇧8 |

Otras teclas se eligen en los ajustes. En Windows lo mismo se inicia con los botones del ratón: los dos a la vez, solos, con Mayús o con Ctrl. Al principio está desactivado, porque mientras el programa vigila los botones cada movimiento del ratón cuesta unos 2 µs de procesador; en reposo, nada. En Wayland la tecla se asigna en los ajustes del sistema.

## Teclas en el editor

| Tecla | Qué hace |
| --- | --- |
| Ctrl+C | copia el área |
| Ctrl+S | guarda el área en `Imágenes/Screenshots` |
| Ctrl+U | sube el área, el enlace va al portapapeles |
| Ctrl+Z | deshace el último dibujo |
| Delete | borra la marca elegida |
| Esc | cierra el editor |

En macOS, ⌘ en lugar de Ctrl.

Un clic elige una marca con cualquier herramienta. Un doble clic o Enter cambia el texto. La rueda del ratón cambia el grosor.

Lo demás también está en los ajustes: qué se graba con el vídeo, la carpeta, el idioma, la subida y el inicio con el sistema, activado hasta que usted lo desactive. Los ajustes se abren desde el menú del icono de la bandeja o iniciando el programa con `--settings`. Iniciarlo por segunda vez sin eso también hace una captura.

## La extensión para Firefox

Captura la parte visible de una página o la página completa y abre el mismo editor en una pestaña nueva: seleccionar, dibujar, copiar, guardar. Las teclas son Alt+Mayús+S y Alt+Mayús+P. Mira una página solo cuando usted lo pide, no envía nada y recuerda solo el idioma. No graba vídeo, y la altura de una captura no pasa de 32 000 píxeles.

## Cuánto ocupa

Medido el 24.11.2026 en Windows Server 2022, pantalla de 1920 × 1080, compilación optimizada:

| Qué | Resultado |
| --- | --- |
| Procesador en reposo | 0 ms en 10 s |
| Memoria en reposo | 14 MB antes de la primera captura, 25 MB después de siete |
| Procesos en reposo | 1 |
| De la tecla al editor | 0,61–0,75 s en siete ejecuciones |
| Tamaño del programa | 5,2 MB |

El editor tarda esto en abrirse porque su ventana se crea para cada captura y se destruye después. Es el precio de la carga nula en reposo.

El vídeo lo graba un proceso aparte, que desaparece con toda su memoria al terminar. En este servidor, sin tarjeta gráfica, 30 fotogramas por segundo de 1920 × 1080 ocupan un núcleo del procesador y 207 MB; los de 800 × 600, 0,4 de un núcleo y 78 MB. Una tarjeta que codifique H.264 asume ese trabajo; aún no se ha medido.

## Estado

| Sistema | Estado |
| --- | --- |
| Windows | Funciona en Windows Server 2022 con un monitor. El vídeo también se ha probado ahí; la grabación de sonido desde dispositivos no, porque el servidor no los tiene. Aún no se ha probado en Windows 10 y 11. |
| macOS 10.13+ | Se compila y pasa las pruebas; en una máquina de GitHub se instala, se inicia y graba la pantalla con el sonido del ordenador. Nadie lo ha probado a mano todavía. |
| Ubuntu 24.04+ | Se compila y pasa las pruebas; en una máquina de GitHub se instala, se inicia y graba la pantalla. En una pantalla X11 virtual de Ubuntu 26.04 funcionan las teclas, la selección, el guardado y el vídeo; en Wayland no se ha comprobado. Nadie lo ha probado a mano todavía. En Wayland, asigne el atajo en los ajustes del teclado: la orden es `just-screenshotter`. |
| Firefox 128+ | Funciona en Firefox 161, probada sin ventana. Mozilla aún no la ha firmado, así que hay que cargarla de nuevo tras reiniciar Firefox. |

## Instalación

| Sistema | Archivo | Cómo instalarlo |
| --- | --- | --- |
| Windows 10 y 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Ejecútelo. Si Windows no conoce el programa: «Más información», «Ejecutar de todas formas». |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Arrastre el programa a Aplicaciones y ábralo. Cuando macOS se niegue, vaya a Ajustes del Sistema, «Privacidad y seguridad», y pulse «Abrir igualmente». |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | Abra `about:debugging`, «Este Firefox», «Cargar complemento temporal», y elija el archivo. Tras reiniciar Firefox, cárguelo de nuevo. |

GitHub compila estos archivos a partir del código de este repositorio, instala cada uno, inicia el programa y solo entonces los publica. No llevan firma de pago, por eso Windows y macOS avisan en el primer inicio.

También puede compilar el programa usted mismo. Hacen falta Node.js 24 o posterior y Rust; en Windows, también Microsoft C++ Build Tools; en Ubuntu, también `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

El instalador aparece en `src-tauri/target/release/bundle`.

## Traducciones

Las traducciones son automáticas y no las ha revisado ningún hablante nativo. Si ve un error, corríjalo en `locales` o en `docs`.

## Participar y licencia

Cómo compilar, probar y cambiar el programa: [CONTRIBUTING.md](../CONTRIBUTING.md). La licencia es [MIT](../LICENSE).

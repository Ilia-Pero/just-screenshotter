<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Una herramienta de capturas sin carga en reposo para Windows, macOS y Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · **Español** · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md)

![El editor sobre una pantalla de muestra: un marcador, un marco, una flecha y un texto](editor.png)

Pulse Print Screen, seleccione un área, dibuje sobre ella y copie, guarde o suba la imagen. Entre capturas el programa solo espera la tecla: no tiene ventanas ni temporizadores y no ocupa el procesador.

## Qué hace

- Captura todos los monitores y abre el editor encima de ellos.
- Dibuja con lápiz, línea, flecha, marco, marcador y texto en ocho colores.
- Copia la imagen, la guarda en PNG o la sube a [su propio servidor](../server/README.md) y copia el enlace.
- Habla 12 idiomas. Al principio usa el del sistema; otro se elige en el menú.
- No envía nada por su cuenta: no hay cuenta, telemetría ni búsqueda de actualizaciones. Solo usa la red para subir una imagen al servidor que usted indique.

## Teclas en el editor

| Tecla | Qué hace |
| --- | --- |
| Ctrl+C | copia el área |
| Ctrl+S | guarda el área en `Imágenes/Screenshots` |
| Ctrl+U | sube el área, el enlace va al portapapeles |
| Ctrl+Z | deshace el último dibujo |
| Esc | cierra el editor |

En macOS, ⌘ en lugar de Ctrl.

En el menú del icono de la bandeja están el atajo, la carpeta, el idioma, la dirección del servidor y el inicio con el sistema. Iniciar el programa por segunda vez también hace una captura.

## Cuánto ocupa

Medido el 24.11.2026 en Windows Server 2022, pantalla de 1920 × 1080, compilación optimizada:

| Qué | Resultado |
| --- | --- |
| Procesador en reposo | 0 ms en 10 s |
| Memoria en reposo | 14 MB antes de la primera captura, 25 MB después de siete |
| Procesos en reposo | 1 |
| De la tecla al editor | 0,61–0,75 s en siete ejecuciones |
| Tamaño del programa | 5,0 MB |

El editor tarda esto en abrirse porque su ventana se crea para cada captura y se destruye después. Es el precio de la carga nula en reposo.

## Estado

| Sistema | Estado |
| --- | --- |
| Windows | Funciona en Windows Server 2022 con un monitor. Aún no se ha probado en Windows 10 y 11. |
| macOS 10.13+ | Se compila y pasa las pruebas. Nadie lo ha ejecutado todavía. |
| Ubuntu 24.04+ | Se compila y pasa las pruebas. Nadie lo ha ejecutado todavía. En Wayland, asigne el atajo en los ajustes del teclado: la orden es `just-screenshotter`. |

## Instalación

Aún no hay descargas listas, así que hay que compilar el programa. Hacen falta Node.js 24 o posterior y Rust; en Windows, también Microsoft C++ Build Tools; en Ubuntu, también `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

El instalador aparece en `src-tauri/target/release/bundle`. No está firmado, así que Windows y macOS preguntarán si confía en él.

## Traducciones

Las traducciones son automáticas y no las ha revisado ningún hablante nativo. Si ve un error, corríjalo en `locales` o en `docs`.

## Participar y licencia

Cómo compilar, probar y cambiar el programa: [CONTRIBUTING.md](../CONTRIBUTING.md). La licencia es [MIT](../LICENSE).

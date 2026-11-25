<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Capturas y grabación de pantalla para Windows, macOS y Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · **Español** · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![El editor sobre una pantalla de ejemplo: marcador, óvalo, flecha y texto](editor.png)

Al pulsar Print Screen, el programa captura todos los monitores y abre un editor encima de la pantalla. Allí se selecciona un área, se dibuja sobre ella y se copia, guarda o envía la imagen. La misma área puede grabarse en vídeo. Entre capturas queda en memoria un proceso con un icono en la bandeja.

## Instalación

| Sistema | Archivo | Cómo instalarlo |
| --- | --- | --- |
| Windows 10, 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Ejecute el archivo. Si Windows muestra una advertencia, elija «Más información» y después «Ejecutar de todas formas». |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Arrastre el programa a Aplicaciones y ábralo. Después del primer rechazo de macOS, abra Ajustes del Sistema, «Privacidad y seguridad», y pulse «Abrir igualmente». |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | Abra `about:debugging`, «Este Firefox», «Cargar complemento temporal» y elija el archivo. Después de reiniciar Firefox hay que cargarlo de nuevo. |
| Chrome, Edge, Opera, Yandex Browser | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | Descomprima el archivo en una carpeta que vaya a conservar. Abra `chrome://extensions`, active el modo de desarrollador, pulse «Cargar extensión sin empaquetar» y elija la carpeta. |

Windows y macOS avisan porque los archivos no tienen una firma de pago. GitHub los compila a partir de este repositorio, instala los programas en equipos limpios y los inicia antes de publicar una versión.

## Funciones

- Dibujo con lápiz, línea, flecha, marco, óvalo, marcador y texto: ocho colores, grosor de 1 a 72 píxeles.
- Lo dibujado se puede mover, cambiar de color o tamaño, volver a escribir o borrar en cualquier momento.
- La imagen se copia, se guarda en PNG o se envía a [su propio servidor](../server/README.md).
- Vídeo de un área con el sonido del ordenador y el micrófono: MP4 en Windows, MOV en macOS, WebM en Linux. Antes de macOS 13 solo se graba el micrófono.
- 14 idiomas; las teclas y la carpeta se eligen en los ajustes.

## Teclas

| Acción | Windows, Linux | macOS |
| --- | --- | --- |
| Captura de un área | Print Screen | ⌘⇧9 |
| Captura de toda la pantalla | Shift + Print Screen | |
| Captura de la ventana activa, solo Windows | Alt + Print Screen | |
| Vídeo de un área | Ctrl + Shift + Print Screen | ⌘⇧8 |

Las teclas se cambian en los ajustes. En el editor, Ctrl+C copia, Ctrl+S guarda, Ctrl+Z deshace y Esc cierra.

## Extensión del navegador

Captura la parte visible de una página (Alt+Shift+S) o la página entera (Alt+Shift+P) y abre el mismo editor en una pestaña nueva. Funciona en Firefox y en navegadores basados en Chromium.

## Datos y red

Las capturas y grabaciones se quedan en su ordenador. El programa usa la red cuando envía una captura a su servidor y cuando pulsa «Buscar actualizaciones». No hay cuenta ni telemetría.

## Más información

Las mediciones, los sistemas comprobados y la compilación se describen [en inglés](../README.md).

Este texto es una traducción automática que no han revisado hablantes nativos. Las correcciones van en la carpeta `docs`.

[MIT](../LICENSE)

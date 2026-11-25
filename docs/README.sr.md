<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Снимци и снимање екрана за Windows, macOS и Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · **Српски** · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![Уређивач преко пробног екрана: маркер, овал, стрелица и натпис](editor.png)

На тастер Print Screen програм снима све мониторе и преко њих отвара уређивач. У њему се бира област, по њој се црта, а затим се слика копира, чува или шаље. Иста област може да се сними и као видео. Између снимака у меморији остаје један процес са иконицом у системској палети.

## Инсталација

| Систем | Датотека | Како се инсталира |
| --- | --- | --- |
| Windows 10, 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Покрените датотеку. Ако Windows прикаже упозорење, изаберите „Више информација“, па „Ипак покрени“. |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Превуците програм у „Апликације“ и отворите га. Када га macOS први пут одбије, отворите системска подешавања, „Приватност и безбедност“, и притисните „Ипак отвори“. |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | Отворите `about:debugging`, „Овај Firefox“, „Учитај привремени додатак“ и изаберите датотеку. После поновног покретања Firefox-а учитајте је опет. |
| Chrome, Edge, Opera, Yandex Browser | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | Распакујте архиву у фасциклу коју нећете брисати. Отворите `chrome://extensions`, укључите режим за програмере, притисните „Учитај распаковано“ и изаберите фасциклу. |

Windows и macOS упозоравају јер датотеке немају плаћени потпис. GitHub их прави из овог репозиторијума, инсталира програме на чистим машинама и покреће их пре објављивања издања.

## Могућности

- Цртање оловком, линијом, стрелицом, оквиром, овалом, маркером и текст: осам боја, дебљина од 1 до 72 пиксела.
- Нацртано може у сваком тренутку да се помери, обоји, промени у величини, поново откуца или обрише.
- Слика се копира, чува као PNG или шаље на [ваш сервер](../server/README.md).
- Видео области са звуком рачунара и микрофоном: MP4 на Windows-у, MOV на macOS-у, WebM на Linux-у. Пре macOS 13 снима се само микрофон.
- 14 језика; тастери и фасцикла бирају се у подешавањима.

## Тастери

| Радња | Windows, Linux | macOS |
| --- | --- | --- |
| Снимак области | Print Screen | ⌘⇧9 |
| Снимак целог екрана | Shift + Print Screen | |
| Снимак активног прозора, само Windows | Alt + Print Screen | |
| Видео области | Ctrl + Shift + Print Screen | ⌘⇧8 |

Тастери се мењају у подешавањима. У уређивачу Ctrl+C копира, Ctrl+S чува, Ctrl+Z поништава, Esc затвара.

## Додатак за прегледач

Снима видљиви део странице (Alt+Shift+S) или целу страницу (Alt+Shift+P) и отвара исти уређивач у новој картици. Ради у Firefox-у и у прегледачима заснованим на Chromium-у.

## Подаци и мрежа

Снимци остају на вашем рачунару. Програм користи мрежу када шаљете снимак на свој сервер и када притиснете „Провери ажурирања“. Нема налога ни телеметрије.

## Више

Мерења, проверени системи и прављење програма описани су [на енглеском](../README.md).

Овај текст је машински превод који изворни говорници нису проверили. Исправке иду у фасциклу `docs`.

[MIT](../LICENSE)

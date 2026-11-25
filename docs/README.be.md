<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Скрыншоты і запіс экрана для Windows, macOS і Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · **Беларуская** · [Қазақша](README.kk.md)

![Рэдактар паверх пробнага экрана: маркер, авал, стрэлка і подпіс](editor.png)

Па клавішы Print Screen праграма здымае ўсе маніторы і адкрывае паверх экрана рэдактар. У ім вылучаюць вобласць, малююць на ёй, а потым капіруюць, захоўваюць або адпраўляюць здымак. Тую ж вобласць можна запісаць на відэа. Паміж здымкамі ў памяці застаецца адзін працэс са значком у трэі.

## Усталяванне

| Сістэма | Файл | Як усталяваць |
| --- | --- | --- |
| Windows 10, 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Запусціце файл. Калі Windows пакажа папярэджанне, націсніце «Падрабязней», потым «Выканаць у любым выпадку». |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Перацягніце праграму ў «Праграмы» і адкрыйце. Пасля першай адмовы macOS адкрыйце сістэмныя налады, «Канфідэнцыяльнасць і бяспека», і націсніце «Усё роўна адкрыць». |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | Адкрыйце `about:debugging`, «Гэты Firefox», «Загрузіць часовае дапаўненне» і выберыце файл. Пасля перазапуску Firefox загрузіце яго зноў. |
| Chrome, Edge, Opera, Яндэкс Браўзер | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | Распакуйце архіў у папку, якую не будзеце выдаляць. Адкрыйце `chrome://extensions`, уключыце рэжым распрацоўшчыка, націсніце «Загрузіць распакаванае пашырэнне» і выберыце папку. |

Windows і macOS папярэджваюць пра файлы, бо ў іх няма платнага подпісу. GitHub збірае файлы з гэтага рэпазіторыя, усталёўвае праграмы на чыстых машынах і запускае іх, і толькі потым публікуе выпуск.

## Магчымасці

- Малюнак алоўкам, лініяй, стрэлкай, рамкай, авалам, маркерам і тэкст: восем колераў, таўшчыня ад 1 да 72 пікселяў.
- Намаляванае можна ў любы момант перасунуць, перафарбаваць, змяніць у памеры, перадрукаваць або выдаліць.
- Здымак капіруецца, захоўваецца ў PNG або адпраўляецца на [ваш сервер](../server/README.md).
- Відэа вобласці з гукам камп’ютара і мікрафонам: MP4 на Windows, MOV на macOS, WebM на Linux. Да macOS 13 запісваецца толькі мікрафон.
- 14 моў, свае клавішы і папка для файлаў у наладах.

## Клавішы

| Дзеянне | Windows, Linux | macOS |
| --- | --- | --- |
| Скрыншот вобласці | Print Screen | ⌘⇧9 |
| Скрыншот усяго экрана | Shift + Print Screen | |
| Скрыншот актыўнага акна, толькі Windows | Alt + Print Screen | |
| Відэа вобласці | Ctrl + Shift + Print Screen | ⌘⇧8 |

Клавішы мяняюць у наладах. У рэдактары Ctrl+C капіруе, Ctrl+S захоўвае, Ctrl+Z адмяняе, Esc закрывае.

## Пашырэнне для браўзера

Здымае бачную частку старонкі (Alt+Shift+S) або ўсю старонку (Alt+Shift+P) і адкрывае той жа рэдактар у новай укладцы. Працуе ў Firefox і ў браўзерах на Chromium.

## Даныя і сетка

Здымкі і запісы захоўваюцца на вашым камп’ютары. Праграма звяртаецца да сеткі, калі вы адпраўляеце здымак на свой сервер і калі націскаеце «Праверыць абнаўленні». Уліковага запісу і тэлеметрыі няма.

## Падрабязней

Вымярэнні, спіс правераных сістэм і зборка апісаны [па-англійску](../README.md) і [па-руску](README.ru.md).

Гэты тэкст перакладзены машынай, носьбіты мовы яго не правяралі. Памылкі можна выправіць у папцы `docs`.

[MIT](../LICENSE)

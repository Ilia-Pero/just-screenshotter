<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Праграма для здымкаў экрана без нагрузкі ў спакоі для Windows, macOS і Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · **Беларуская** · [Қазақша](README.kk.md)

![Рэдактар паверх пробнага экрана: маркер, рамка, стрэлка і подпіс](editor.png)

Націсніце Print Screen, вылучыце вобласць, намалюйце на ёй што трэба і скапіюйце, захавайце або адпраўце здымак. Тую ж вобласць можна запісаць на відэа. Паміж здымкамі праграма толькі чакае клавішу: у яе няма акон і таймераў, працэсар яна не займае.

## Што ўмее

- Здымае ўсе маніторы і адкрывае рэдактар паверх іх.
- Малюе алоўкам, лініяй, стрэлкай, рамкай, маркерам і піша тэкст, восем колераў.
- Мяняе ўжо намаляванае: пстрычка выбірае пазнаку, пасля гэтага яе або яе канцы можна перацягнуць, задаць таўшчыню і колер, перадрукаваць тэкст або выдаліць.
- Капіюе здымак, захоўвае ў PNG або адпраўляе на [ваш сервер](../server/README.md) і капіюе спасылку.
- Запісвае вобласць на відэа са гукам: MP4 на Windows, MOV на macOS, WebM на Linux. Ці пісаць гук камп’ютара і мікрафон, выбіраеце вы; на macOS да 13-й версіі толькі мікрафон. Запіс спыняюць кнопка «Спыніць», клавіша здымкі або значок у трэі.
- Гаворыць на 14 мовах. Мову выбіраюць у наладах і ва ўсталёўшчыку для Windows, а да выбару гэта мова сістэмы.
- Сама нічога нікуды не адпраўляе: няма ўліковага запісу, тэлеметрыі і праверкі абнаўленняў. Сетка патрэбна толькі для адпраўкі здымка на сервер, які вы ўказалі.

## Клавішы і мыш

| Што робіць | Windows і Linux | macOS |
| --- | --- | --- |
| Здымак вобласці | Print Screen | ⌘⇧9 |
| Здымак усяго экрана | Shift + Print Screen | — |
| Здымак актыўнага акна (Windows) | Alt + Print Screen | — |
| Відэа вобласці | Ctrl + Shift + Print Screen | ⌘⇧8 |

Іншыя клавішы выбіраюць у наладах. На Windows тое самае запускаюць кнопкі мышы: дзве адразу, асобна, з Shift або з Ctrl. Спачатку гэта выключана: пакуль праграма сочыць за кнопкамі, кожны рух мышы займае каля 2 мкс працэсарнага часу, у спакоі нічога. На Wayland клавішу прызначаюць у наладах сістэмы.

## Клавішы ў рэдактары

| Клавіша | Што робіць |
| --- | --- |
| Ctrl+C | капіюе вобласць |
| Ctrl+S | захоўвае вобласць у `Выявы/Screenshots` |
| Ctrl+U | адпраўляе вобласць, спасылка трапляе ў буфер абмену |
| Ctrl+Z | адмяняе апошні малюнак |
| Delete | выдаляе выбраную пазнаку |
| Esc | закрывае рэдактар |

На macOS замест Ctrl клавіша ⌘.

Пстрычка выбірае пазнаку з любым інструментам. Двайная пстрычка або Enter мяняе тэкст. Кола мышы мяняе таўшчыню.

Астатняе таксама ў наладах: што пісаць разам з відэа, папка, мова, адпраўка і запуск разам з сістэмай, які ўключаны, пакуль вы яго не выключыце. Налады адкрываюць з меню значка ў трэі або запускам праграмы з `--settings`. Паўторны запуск без яго таксама робіць здымак.

## Пашырэнне для Firefox

Здымае бачную частку старонкі або ўсю старонку і адкрывае той жа рэдактар у новай укладцы: вылучыць, намаляваць, скапіяваць, захаваць. Клавішы Alt+Shift+S і Alt+Shift+P. Глядзіць на старонку, толькі калі вы папросіце, нікуды нічога не адпраўляе і памятае толькі мову. Відэа не піша, вышыня здымка да 32 000 пікселяў.

## Наколькі яна лёгкая

Замеры 24.11.2026 на Windows Server 2022, экран 1920 × 1080, аптымізаваная зборка:

| Што вымяралі | Вынік |
| --- | --- |
| Працэсар у спакоі | 0 мс за 10 с |
| Памяць у спакоі | 14 МБ да першага здымка, 25 МБ пасля сямі |
| Працэсаў у спакоі | 1 |
| Ад націску клавішы да рэдактара | 0,61–0,75 с у сямі выкліках |
| Памер праграмы | 5,2 МБ |

Рэдактар адкрываецца так доўга, бо яго акно ствараецца пры кожным здымку і знішчаецца пасля яго. Гэта плата за нулявую нагрузку ў спакоі.

Відэа піша асобны працэс. Калі запіс канчаецца, ён завяршаецца, і ўся яго памяць вяртаецца сістэме. На гэтым серверы, дзе відэакарты няма, 30 кадраў у секунду пры 1920 × 1080 займаюць адно ядро працэсара і 207 МБ, пры 800 × 600 займаюць 0,4 ядра і 78 МБ. Відэакарта, якая кадзіруе H.264, бярэ гэту працу на сябе; гэта яшчэ не вымяралі.

## Стан

| Сістэма | Што вядома |
| --- | --- |
| Windows | Працуе на Windows Server 2022 з адным маніторам. Відэа там таксама праверана, а запіс гуку з прылад не: на серверы іх няма. На Windows 10 і 11 яшчэ не правяралі. |
| macOS 10.13+ | Збіраецца, тэсты праходзяць; на машыне GitHub усталёўваецца, запускаецца і піша экран з гукам камп’ютара. Рукамі яе яшчэ ніхто не правяраў. |
| Ubuntu 24.04+ | Збіраецца, тэсты праходзяць; на машыне GitHub усталёўваецца, запускаецца і піша экран. На віртуальным экране X11 у Ubuntu 26.04 працуюць клавішы, вылучэнне, захаванне і відэа, на Wayland не правяралі. Рукамі яе яшчэ ніхто не правяраў. На Wayland клавішу прызначаюць у наладах клавіятуры: каманда `just-screenshotter`. |
| Firefox 128+ | Працуе ў Firefox 161, праверана без акна. Mozilla яго яшчэ не падпісала, таму пасля перазапуску Firefox яго загружаюць зноў. |

## Усталяванне

| Сістэма | Файл | Як усталяваць |
| --- | --- | --- |
| Windows 10 і 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Запусціце яго. Калі Windows не ведае праграму: «Падрабязней», «Выканаць у любым выпадку». |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Перацягніце праграму ў Applications і адкрыйце. Калі macOS адмовіць, адкрыйце System Settings, Privacy & Security, і націсніце Open Anyway. |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | Адкрыйце `about:debugging`, This Firefox, Load Temporary Add-on і выберыце файл. Пасля перазапуску Firefox загрузіце яго зноў. |

GitHub збірае гэтыя файлы з кода гэтага рэпазіторыя, усталёўвае кожны, запускае праграму і толькі потым публікуе. Платнага подпісу ў файлаў няма, таму пры першым запуску Windows і macOS папярэджваюць.

Праграму можна сабраць і самому. Спатрэбяцца Node.js 24 або навейшы і Rust, на Windows яшчэ Microsoft C++ Build Tools, на Ubuntu яшчэ `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

Усталёўшчык з’явіцца ў `src-tauri/target/release/bundle`.

## Пераклады

Асноўныя тэксты рускія і англійскія. Астатнія пераклады машынныя, носьбіты мовы іх не правяралі. Калі бачыце памылку, выпраўце яе ў `locales` або ў `docs`.

## Удзел і ліцэнзія

Як сабраць, праверыць і змяніць праграму: [CONTRIBUTING.md](../CONTRIBUTING.md). Ліцэнзія [MIT](../LICENSE).

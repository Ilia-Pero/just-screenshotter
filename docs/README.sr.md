<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Програм за снимке екрана без оптерећења у мировању за Windows, macOS и Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · **Српски** · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md)

![Уређивач преко пробног екрана: маркер, оквир, стрелица и текст](editor.png)

Притисните Print Screen, изаберите област, цртајте по њој, па слику копирајте, сачувајте или отпремите. Између снимака програм само чека тастер: нема прозора ни тајмера и не заузима процесор.

## Шта ради

- Снима све мониторе и отвара уређивач преко њих.
- Црта оловком, линијом, стрелицом, оквиром, маркером и текстом у осам боја.
- Копира слику, чува је као PNG или је отпрема на [ваш сервер](../server/README.md) и копира везу.
- Говори 12 језика. На почетку језик система, а други се бира у менију.
- Сам не шаље ништа: нема налога, телеметрије ни провере ажурирања. Мрежу користи само да отпреми слику на сервер који сте задали.

## Тастери у уређивачу

| Тастер | Шта ради |
| --- | --- |
| Ctrl+C | копира област |
| Ctrl+S | чува област у `Слике/Screenshots` |
| Ctrl+U | отпрема област, веза иде у клипборд |
| Ctrl+Z | поништава последњи цртеж |
| Esc | затвара уређивач |

На macOS-у ⌘ уместо Ctrl.

У менију иконе у траци су пречица, фасцикла, језик, адреса сервера и покретање са системом. Поновно покретање програма такође прави снимак.

## Колико је лаган

Измерено 24.11.2026. на Windows Server 2022, екран 1920 × 1080, оптимизована верзија:

| Шта | Резултат |
| --- | --- |
| Процесор у мировању | 0 ms за 10 s |
| Меморија у мировању | 14 MB пре првог снимка, 25 MB после седам |
| Процеса у мировању | 1 |
| Од притиска тастера до уређивача | 0,61–0,75 s у седам покретања |
| Величина програма | 5,0 MB |

Уређивачу треба оволико времена јер се његов прозор прави за сваки снимак и уништава после њега. То је цена нултог оптерећења у мировању.

## Стање

| Систем | Стање |
| --- | --- |
| Windows | Ради на Windows Server 2022 са једним монитором. На Windows 10 и 11 још није проверено. |
| macOS 10.13+ | Гради се и пролази тестове. Нико га још није покренуо. |
| Ubuntu 24.04+ | Гради се и пролази тестове. Нико га још није покренуо. На Wayland-у пречицу задајте у подешавањима тастатуре: наредба је `just-screenshotter`. |

## Инсталација

Готових датотека још нема, па програм треба изградити. Потребни су Node.js 24 или новији и Rust, на Windows-у и Microsoft C++ Build Tools, на Ubuntu-у и `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

Инсталациони програм се појављује у `src-tauri/target/release/bundle`. Није потписан, па ће Windows и macOS питати да ли му верујете.

## Преводи

Преводи су машински и изворни говорници их нису проверили. Ако видите грешку, исправите је у `locales` или у `docs`.

## Учешће и лиценца

Како изградити, проверити и мењати програм: [CONTRIBUTING.md](../CONTRIBUTING.md). Лиценца је [MIT](../LICENSE).

<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Програм за снимке екрана без оптерећења у мировању за Windows, macOS и Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · **Српски** · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![Уређивач преко пробног екрана: маркер, оквир, стрелица и текст](editor.png)

Притисните Print Screen, изаберите област, цртајте по њој, па слику копирајте, сачувајте или отпремите, или снимите област као видео. Између снимака програм само чека тастер: нема прозора ни тајмера и не заузима процесор.

## Шта ради

- Снима све мониторе и отвара уређивач преко њих.
- Црта оловком, линијом, стрелицом, оквиром, маркером и текстом у осам боја.
- Мења већ нацртано: клик бира ознаку, а затим се она или њени крајеви превлаче, мењају се дебљина и боја, текст се куца поново или се ознака брише.
- Копира слику, чува је као PNG или је отпрема на [ваш сервер](../server/README.md) и копира везу.
- Снима област као видео са звуком: MP4 на Windows-у, MOV на macOS-у, WebM на Linux-у. Ви бирате да ли се снимају звук рачунара и микрофон; на macOS-у пре верзије 13 само микрофон. Снимање завршавају дугме „Заустави“, пречица или икона у траци.
- Говори 14 језика. Језик се бира у подешавањима и у инсталацији за Windows; до тада је то језик система.
- Сам не шаље ништа: нема налога, телеметрије ни провере ажурирања. Мрежу користи само да отпреми слику на сервер који сте задали.

## Тастери и миш

| Шта раде | Windows и Linux | macOS |
| --- | --- | --- |
| Снимак области | Print Screen | ⌘⇧9 |
| Снимак целог екрана | Shift + Print Screen | — |
| Снимак активног прозора (Windows) | Alt + Print Screen | — |
| Видео области | Ctrl + Shift + Print Screen | ⌘⇧8 |

Друге тастере бирате у подешавањима. На Windows-у исто покрећу и дугмад миша: оба одједном, сама, уз Shift или уз Ctrl. У почетку је то искључено, јер док програм прати дугмад, сваки покрет миша троши око 2 µs процесорског времена; у мировању ништа. На Wayland-у тастер се задаје у подешавањима система.

## Тастери у уређивачу

| Тастер | Шта ради |
| --- | --- |
| Ctrl+C | копира област |
| Ctrl+S | чува област у `Слике/Screenshots` |
| Ctrl+U | отпрема област, веза иде у клипборд |
| Ctrl+Z | поништава последњи цртеж |
| Delete | брише изабрану ознаку |
| Esc | затвара уређивач |

На macOS-у ⌘ уместо Ctrl.

Клик бира ознаку уз било који алат. Двоклик или Enter мења текст. Точкић миша мења дебљину.

И остало је у подешавањима: шта се снима уз видео, фасцикла, језик, отпремање и покретање са системом, које је укључено док га не искључите. Подешавања отвара мени иконе у траци или покретање програма са `--settings`. Поновно покретање без тога такође прави снимак.

## Додатак за Firefox

Снима видљиви део странице или целу страницу и отвара исти уређивач у новој картици: изабрати, цртати, копирати, сачувати. Тастери су Alt+Shift+S и Alt+Shift+P. Гледа страницу само кад то затражите, ништа не шаље и памти само језик. Не снима видео, а висина снимка је до 32.000 пиксела.

## Колико је лаган

Измерено 24.11.2026. на Windows Server 2022, екран 1920 × 1080, оптимизована верзија:

| Шта | Резултат |
| --- | --- |
| Процесор у мировању | 0 ms за 10 s |
| Меморија у мировању | 14 MB пре првог снимка, 25 MB после седам |
| Процеса у мировању | 1 |
| Од притиска тастера до уређивача | 0,61–0,75 s у седам покретања |
| Величина програма | 5,2 MB |

Уређивачу треба оволико времена јер се његов прозор прави за сваки снимак и уништава после њега. То је цена нултог оптерећења у мировању.

Видео снима посебан процес, који на крају снимања нестаје са свом својом меморијом. На овом серверу, без графичке картице, 30 слика у секунди при 1920 × 1080 заузимају једно језгро процесора и 207 MB, а при 800 × 600 0,4 језгра и 78 MB. Графичка картица која кодира H.264 преузима тај посао; то још није мерено.

## Стање

| Систем | Стање |
| --- | --- |
| Windows | Ради на Windows Server 2022 са једним монитором. Видео је тамо такође проверен, а снимање звука са уређаја није, јер их сервер нема. На Windows 10 и 11 још није проверено. |
| macOS 10.13+ | Гради се и пролази тестове; на машини GitHub-а се инсталира, покреће и снима екран са звуком рачунара. Ручно га још нико није испробао. |
| Ubuntu 24.04+ | Гради се и пролази тестове; на машини GitHub-а се инсталира, покреће и снима екран. Ручно га још нико није испробао. На Wayland-у пречицу задајте у подешавањима тастатуре: наредба је `just-screenshotter`. |
| Firefox 128+ | Ради у Firefox-у 161, проверено без прозора. Mozilla га још није потписала, па га после поновног покретања Firefox-а треба поново учитати. |

## Инсталација

| Систем | Датотека | Како се инсталира |
| --- | --- | --- |
| Windows 10 и 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Покрените је. Ако Windows не познаје програм: „Више информација“, „Ипак покрени“. |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Превуците програм у Applications и отворите га. Кад macOS одбије, отворите System Settings, „Privacy & Security“, и притисните „Open Anyway“. |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | Отворите `about:debugging`, „Овај Firefox“, „Учитај привремени додатак“, и изаберите датотеку. После поновног покретања Firefox-а учитајте је поново. |

GitHub гради ове датотеке из кода овог репозиторијума, сваку инсталира, покреће програм, па их тек онда објављује. Немају плаћени потпис, зато Windows и macOS упозоравају при првом покретању.

Програм можете изградити и сами. Потребни су Node.js 24 или новији и Rust, на Windows-у и Microsoft C++ Build Tools, на Ubuntu-у и `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

Инсталациони програм се појављује у `src-tauri/target/release/bundle`.

## Преводи

Преводи су машински и изворни говорници их нису проверили. Ако видите грешку, исправите је у `locales` или у `docs`.

## Учешће и лиценца

Како изградити, проверити и мењати програм: [CONTRIBUTING.md](../CONTRIBUTING.md). Лиценца је [MIT](../LICENSE).

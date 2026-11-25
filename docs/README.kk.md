<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Windows, macOS және Linux үшін скриншоттар мен экран жазбасы</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · **Қазақша**

![Үлгі экран үстіндегі редактор: маркер, сопақ, көрсеткі және жазу](editor.png)

Print Screen пернесі басылғанда бағдарлама барлық мониторды түсіріп, олардың үстінен редактор ашады. Онда аймақты таңдап, үстіне сурет салып, суретті көшіреді, сақтайды немесе жібереді. Сол аймақты бейнеге жазуға болады. Түсірілімдер арасында жадта трей белгішесі бар бір процесс қалады.

## Орнату

| Жүйе | Файл | Қалай орнатылады |
| --- | --- | --- |
| Windows 10, 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Файлды іске қосыңыз. Windows ескерту көрсетсе, «Толығырақ», содан кейін «Бәрібір іске қосу» дегенді таңдаңыз. |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Бағдарламаны «Бағдарламалар» қалтасына сүйреп апарып, ашыңыз. macOS бірінші рет бас тартқан соң жүйе баптауларын, «Құпиялылық және қауіпсіздік» бөлімін ашып, «Бәрібір ашу» түймесін басыңыз. |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | `about:debugging` бетін, «Осы Firefox» бөлімін ашып, уақытша қосымшаны жүктеу түймесін басыңыз да, файлды таңдаңыз. Firefox қайта іске қосылған соң оны қайта жүктеңіз. |
| Chrome, Edge, Opera, Yandex Browser | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | Мұрағатты өшірмейтін қалтаға шығарыңыз. `chrome://extensions` бетін ашып, әзірлеуші режимін қосыңыз, қапталмаған кеңейтімді жүктеу түймесін басып, қалтаны таңдаңыз. |

Файлдарда ақылы қолтаңба болмағандықтан, Windows пен macOS ескерту көрсетеді. GitHub файлдарды осы репозиторийден құрастырады, бағдарламаларды таза машиналарға орнатып, іске қосады, содан кейін ғана шығарылымды жариялайды.

## Мүмкіндіктер

- Қарындаш, сызық, көрсеткі, жақтау, сопақ, маркер және мәтін: сегіз түс, қалыңдығы 1-ден 72 пиксельге дейін.
- Салынған белгіні кез келген уақытта жылжытуға, бояуға, өлшемін өзгертуге, қайта теруге немесе жоюға болады.
- Сурет көшіріледі, PNG түрінде сақталады немесе [өз серверіңізге](../server/README.md) жіберіледі.
- Аймақ бейнесі компьютер дыбысымен және микрофонмен: Windows-та MP4, macOS-та MOV, Linux-та WebM. macOS 13-ке дейін тек микрофон жазылады.
- 14 тіл; пернелер мен қалта баптауларда таңдалады.

## Пернелер

| Әрекет | Windows, Linux | macOS |
| --- | --- | --- |
| Аймақтың скриншоты | Print Screen | ⌘⇧9 |
| Бүкіл экранның скриншоты | Shift + Print Screen | |
| Белсенді терезенің скриншоты, тек Windows | Alt + Print Screen | |
| Аймақтың бейнесі | Ctrl + Shift + Print Screen | ⌘⇧8 |

Пернелер баптауларда өзгертіледі. Редакторда Ctrl+C көшіреді, Ctrl+S сақтайды, Ctrl+Z болдырмайды, Esc жабады.

## Браузер кеңейтімі

Беттің көрінетін бөлігін (Alt+Shift+S) немесе бүкіл бетті (Alt+Shift+P) түсіріп, сол редакторды жаңа қойындыда ашады. Firefox-та және Chromium негізіндегі браузерлерде жұмыс істейді.

## Деректер және желі

Суреттер мен жазбалар компьютеріңізде қалады. Бағдарлама желіні суретті серверіңізге жібергенде және «Жаңартуларды тексеру» түймесін басқанда ғана пайдаланады. Тіркелгі мен телеметрия жоқ.

## Толығырақ

Өлшемдер, тексерілген жүйелер және құрастыру [ағылшын тілінде](../README.md) сипатталған.

Бұл мәтінді машина аударды, тіл иелері тексерген жоқ. Түзетулер `docs` қалтасына енгізіледі.

[MIT](../LICENSE)

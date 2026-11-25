<div dir="rtl">

<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">צילומי מסך והקלטת מסך עבור Windows‏, macOS ו־Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · **עברית** · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![העורך מעל מסך לדוגמה: מרקר, אליפסה, חץ וכיתוב](editor.png)

בלחיצה על Print Screen התוכנה מצלמת את כל המסכים ופותחת מעליהם עורך. בעורך בוחרים אזור, מציירים עליו ואז מעתיקים, שומרים או שולחים את התמונה. את אותו אזור אפשר להקליט כווידאו. בין צילום לצילום נשאר בזיכרון תהליך אחד עם סמל במגש המערכת.

## התקנה

| מערכת | קובץ | איך מתקינים |
| --- | --- | --- |
| Windows 10, 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | הפעילו את הקובץ. אם Windows מציגה אזהרה, בחרו „מידע נוסף” ואז „הפעל בכל זאת”. |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | גררו את התוכנה לתיקיית „יישומים” ופתחו אותה. אחרי הסירוב הראשון של macOS פתחו את הגדרות המערכת, „פרטיות ואבטחה”, ולחצו על „פתח בכל זאת”. |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | פתחו את `about:debugging`, „Firefox זה”, „טעינת תוספת זמנית” ובחרו את הקובץ. אחרי הפעלה מחדש של Firefox טענו אותו שוב. |
| Chrome, Edge, Opera, Yandex Browser | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | חלצו את הארכיון לתיקייה שתישאר במקומה. פתחו את `chrome://extensions`, הפעילו את מצב המפתחים, לחצו על „טעינת פריט unpacked” ובחרו את התיקייה. |

Windows ו־macOS מציגות אזהרה משום שלקבצים אין חתימה בתשלום. GitHub בונה אותם מהמאגר הזה, מתקין את התוכנות במחשבים נקיים ומפעיל אותן לפני פרסום גרסה.

## אפשרויות

- ציור בעיפרון, קו, חץ, מסגרת, אליפסה, מרקר וטקסט: שמונה צבעים, עובי של 1 עד 72 פיקסלים.
- את מה שצויר אפשר בכל רגע להזיז, לצבוע מחדש, לשנות בגודל, להקליד מחדש או למחוק.
- התמונה מועתקת, נשמרת כ־PNG או נשלחת אל [השרת שלכם](../server/README.md).
- וידאו של אזור עם צליל המחשב והמיקרופון: MP4 ב־Windows,‏ MOV ב־macOS,‏ WebM ב־Linux. לפני macOS 13 מוקלט רק המיקרופון.
- 14 שפות; את המקשים ואת התיקייה קובעים בהגדרות.

## מקשים

| פעולה | Windows, Linux | macOS |
| --- | --- | --- |
| צילום אזור | Print Screen | ⌘⇧9 |
| צילום כל המסך | Shift + Print Screen | |
| צילום החלון הפעיל, Windows בלבד | Alt + Print Screen | |
| וידאו של אזור | Ctrl + Shift + Print Screen | ⌘⇧8 |

את המקשים משנים בהגדרות. בעורך Ctrl+C מעתיק, Ctrl+S שומר, Ctrl+Z מבטל, Esc סוגר.

## תוסף לדפדפן

מצלם את החלק הנראה של הדף (Alt+Shift+S) או את הדף כולו (Alt+Shift+P) ופותח את אותו עורך בלשונית חדשה. פועל ב־Firefox ובדפדפנים מבוססי Chromium.

## נתונים ורשת

הצילומים וההקלטות נשארים במחשב שלכם. התוכנה פונה לרשת כשאתם שולחים צילום אל השרת שלכם וכשאתם לוחצים על „בדיקת עדכונים”. אין חשבון ואין טלמטריה.

## עוד

המדידות, המערכות שנבדקו ובניית התוכנה מתוארות [באנגלית](../README.md).

הטקסט תורגם במכונה ולא נבדק בידי דוברי השפה. תיקונים נעשים בתיקייה `docs`.

[MIT](../LICENSE)

</div>

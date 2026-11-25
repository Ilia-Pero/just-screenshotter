<div dir="rtl">

<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">כלי לצילומי מסך ללא עומס במנוחה עבור Windows‏, macOS ו־Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · **עברית** · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md)

![העורך מעל מסך לדוגמה: מרקר, מסגרת, חץ וכיתוב](editor.png)

לחצו על Print Screen, בחרו אזור, ציירו עליו והעתיקו, שמרו או העלו את התמונה, או הקליטו את האזור כווידאו. בין צילום לצילום התוכנה רק ממתינה למקש: אין לה חלונות ואין טיימרים, והיא אינה מעסיקה את המעבד.

## מה היא עושה

- מצלמת את כל המסכים ופותחת מעליהם את העורך.
- מציירת בעיפרון, קו, חץ, מסגרת, מרקר וטקסט בשמונה צבעים.
- משנה את מה שכבר צויר: לחיצה בוחרת סימון, ואז אפשר לגרור אותו או את קצותיו, לקבוע עובי וצבע, להקליד את הטקסט מחדש או למחוק.
- מעתיקה את התמונה, שומרת אותה כ־PNG או מעלה אותה אל [השרת שלכם](../server/README.md) ומעתיקה את הקישור.
- מקליטה את האזור כווידאו עם צליל: MP4 ב־Windows,‏ MOV ב־macOS,‏ WebM ב־Linux. אתם בוחרים אם להקליט את הצליל של המחשב ואת המיקרופון; ב־macOS לעת עתה רק את המיקרופון. ההקלטה נעצרת בכפתור, בקיצור המקשים או בסמל שבמגש.
- מדברת 12 שפות. את השפה בוחרים בעורך, בתפריט הסמל ובתוכנית ההתקנה ל־Windows; עד אז זו שפת המערכת.
- אינה שולחת דבר מעצמה: אין חשבון, אין טלמטריה ואין בדיקת עדכונים. הרשת משמשת רק להעלאת תמונה אל השרת שהגדרתם.

## מקשים בעורך

| מקש | מה הוא עושה |
| --- | --- |
| Ctrl+C | מעתיק את האזור |
| Ctrl+S | שומר את האזור בתיקייה `Screenshots` שבתיקיית התמונות |
| Ctrl+U | מעלה את האזור, והקישור עובר ללוח |
| Ctrl+Z | מבטל את הציור האחרון |
| Delete | מוחק את הסימון שנבחר |
| Esc | סוגר את העורך |

ב־macOS המקש ⌘ במקום Ctrl.

סימונים נבחרים כשאין כלי פעיל. לחיצה כפולה או Enter משנה טקסט. גלגלת העכבר משנה את העובי.

בתפריט הסמל שבמגש נמצאים קיצור המקשים, התיקייה, השפה, כתובת השרת והפעלה עם המערכת. הפעלה נוספת של התוכנה גם היא מצלמת את המסך.

## כמה היא קלה

נמדד ב־24.11.2026 ב־Windows Server 2022, מסך 1920 × 1080, גרסה ממוטבת:

| מה | תוצאה |
| --- | --- |
| מעבד במנוחה | 0 אלפיות השנייה ב־10 שניות |
| זיכרון במנוחה | 14 MB לפני הצילום הראשון, 25 MB אחרי שבעה |
| תהליכים במנוחה | 1 |
| מלחיצת המקש ועד העורך | 0.61–0.75 שניות בשבע הפעלות |
| גודל התוכנה | 5.2 MB |

העורך נפתח בפרק זמן כזה משום שהחלון שלו נוצר לכל צילום ונהרס אחריו. זה המחיר של אפס עומס במנוחה.

את הווידאו מקליט תהליך נפרד, שמסתיים ומחזיר את כל הזיכרון שלו בסוף ההקלטה. בשרת הזה, שאין בו כרטיס מסך, 30 תמונות בשנייה ב־1920 × 1080 מעסיקות ליבת מעבד אחת ו־207 MB, וב־800 × 600 מעסיקות 0.4 ליבה ו־78 MB. כרטיס מסך שמקודד H.264 לוקח את העבודה הזאת על עצמו; זה עדיין לא נמדד.

## מצב

| מערכת | מצב |
| --- | --- |
| Windows | עובדת ב־Windows Server 2022 עם מסך אחד. גם הווידאו נבדק שם; הקלטת צליל מהתקנים לא נבדקה, כי בשרת אין כאלה. עדיין לא נבדקה ב־Windows 10 ו־11. |
| macOS 10.13+ | נבנית ועוברת את הבדיקות; במחשב של GitHub היא מותקנת, מופעלת ומקליטה את המסך. איש עדיין לא ניסה אותה ביד. |
| Ubuntu 24.04+ | נבנית ועוברת את הבדיקות; במחשב של GitHub היא מותקנת, מופעלת ומקליטה את המסך. איש עדיין לא ניסה אותה ביד. ב־Wayland מגדירים את הקיצור בהגדרות המקלדת: הפקודה היא `just-screenshotter`. |

## התקנה

| מערכת | קובץ | איך מתקינים |
| --- | --- | --- |
| Windows 10 ו־11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | הפעילו אותו. אם Windows אינה מכירה את התוכנה: "מידע נוסף", "הפעל בכל זאת". |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | גררו את התוכנה ליישומים ופתחו אותה. כש־macOS מסרבת, פתחו את הגדרות המערכת, "פרטיות ואבטחה", ולחצו על "פתח בכל זאת". |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |

GitHub בונה את הקבצים האלה מהקוד של מאגר זה, מתקין כל אחד מהם, מפעיל את התוכנה ורק אז מפרסם אותם. אין להם חתימה בתשלום, ולכן Windows ו־macOS מזהירות בהפעלה הראשונה.

אפשר גם לבנות את התוכנה בעצמכם. דרושים Node.js 24 ומעלה ו־Rust, ב־Windows גם Microsoft C++ Build Tools, ב־Ubuntu גם `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

</div>

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

<div dir="rtl">

קובץ ההתקנה מופיע בתיקייה `src-tauri/target/release/bundle`.

## תרגומים

התרגומים נעשו במכונה ולא נבדקו על ידי דוברי השפה. אם ראיתם טעות, תקנו אותה בתיקייה `locales` או `docs`.

## השתתפות ורישיון

איך לבנות, לבדוק ולשנות את התוכנה: [CONTRIBUTING.md](../CONTRIBUTING.md). הרישיון הוא [MIT](../LICENSE).

</div>

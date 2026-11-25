# The extension in catalogs

What to write and what to upload when the extension is published. The files come from a [release](https://github.com/Ilia-Pero/just-screenshotter/releases/latest).

| Catalog | Browsers | File | Registration |
| --- | --- | --- | --- |
| [Firefox Add-ons](https://addons.mozilla.org/developers/) | Firefox | `just-screenshotter-firefox.zip` | a Mozilla account |
| [Microsoft Edge Add-ons](https://partner.microsoft.com/dashboard/microsoftedge) | Edge | `just-screenshotter-chrome.zip` | free, a Microsoft or GitHub account |
| [Opera Add-ons](https://addons.opera.com/developer/) | Opera, Yandex Browser | `just-screenshotter-chrome.zip` | an Opera account |
| [Chrome Web Store](https://chrome.google.com/webstore/devconsole) | Chrome, Yandex Browser, Edge, Opera | `just-screenshotter-chrome.zip` | a fee, once |

Pictures: `logo.png` is 300 × 300, `screen.png` is 1280 × 800.

## Name

Just Screenshotter

## Summary

Takes a picture of a page and opens an editor over it. Sends nothing anywhere.

## Description

Takes a picture of the visible part of a page or of the whole page and opens an editor in a new tab. Select an area, draw on it with a pencil, a line, an arrow, a frame, an oval, a marker or a text, and copy the picture or save it as PNG. Everything drawn can be moved, recolored, resized or deleted later.

The keys are Alt+Shift+S for the visible part and Alt+Shift+P for the whole page. A whole page is put together from pieces while it is scrolled; parts that stay in their place on the screen are left on the first piece only. A picture is no taller than 32,000 pixels.

The extension looks at a page only when you ask it to. It has no account, no telemetry and no server, and it remembers only the language you have chosen. It speaks 14 languages. The code is open, under MIT: https://github.com/Ilia-Pero/just-screenshotter

## Описание

Снимает видимую часть страницы или страницу целиком и открывает редактор в новой вкладке. Выделите область, порисуйте на ней карандашом, линией, стрелкой, рамкой, овалом, маркером или напишите текст и скопируйте снимок или сохраните его в PNG. Всё нарисованное можно потом передвинуть, перекрасить, изменить в размере или удалить.

Клавиши: Alt+Shift+S для видимой части и Alt+Shift+P для всей страницы. Страница целиком собирается из кусков, пока она прокручивается; то, что стоит на экране на месте, остаётся только на первом куске. Высота снимка до 32 000 пикселей.

Расширение смотрит на страницу, только когда вы попросите. У него нет учётной записи, телеметрии и сервера, а помнит оно только выбранный язык. Говорит на 14 языках. Код открыт, лицензия MIT: https://github.com/Ilia-Pero/just-screenshotter

## Single purpose

Taking a picture of the page a person is looking at and drawing on it.

## Why the permissions

| Permission | What for |
| --- | --- |
| `activeTab` | to take a picture of the page after a person presses the button or the key; no other page is seen |
| `scripting` | to measure the page, to scroll it while the whole of it is taken, and to return it to where it was |
| `storage` | to remember the language |
| `clipboardWrite` | to put the picture on the clipboard when a person presses Copy |

No data is collected, and no code is loaded from anywhere: everything is in the package.

## For reviewers

The scripts are bundled from TypeScript by esbuild and are not minified. To build the same files with Node.js 24 on any system:

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run extension
```

The packages are the folders `extension/packed/firefox` and `extension/packed/chrome`. The source is in `extension/src` and `src`.

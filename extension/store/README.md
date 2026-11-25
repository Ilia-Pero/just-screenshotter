# Publishing the extension

Texts and files for extension stores. The packages come from a [release](https://github.com/Ilia-Pero/just-screenshotter/releases/latest). Firefox Add-ons receives them from the release workflow together with the texts of [firefox.json](firefox.json).

| Store | Browsers | File | Registration |
| --- | --- | --- | --- |
| [Firefox Add-ons](https://addons.mozilla.org/developers/) | Firefox | `just-screenshotter-firefox.zip` | a Mozilla account |
| [Microsoft Edge Add-ons](https://partner.microsoft.com/dashboard/microsoftedge) | Edge | `just-screenshotter-chrome.zip` | free, a Microsoft or GitHub account |
| [Opera Add-ons](https://addons.opera.com/developer/) | Opera, Yandex Browser | `just-screenshotter-chrome.zip` | an Opera account |
| [Chrome Web Store](https://chrome.google.com/webstore/devconsole) | Chrome, Edge, Opera, Yandex Browser | `just-screenshotter-chrome.zip` | a one-time fee |

Images: `logo.png` is 300 × 300, `screen.png` is 1280 × 800.

## Name

Just Screenshotter

## Summary

Takes a picture of a page and opens an editor to draw on it. Pictures stay on your computer.

## Description

Captures the visible part of a page or the whole page and opens an editor in a new tab. Select an area, draw on it with a pencil, a line, an arrow, a frame, an oval, a marker or text, then copy the picture or save it as PNG. A drawing can be moved, recolored, resized or deleted at any time.

Shortcuts: Alt+Shift+S for the visible part, Alt+Shift+P for the whole page. The page is scrolled and captured in pieces; elements that stay fixed on the screen are kept on the first piece only. One picture holds up to 32,000 pixels of a page in height.

The extension looks at a page after you press its button or shortcut. Pictures stay on your computer: the extension makes no network requests and stores only the language you choose. 14 languages. The source code is open under the MIT license.

<https://github.com/Ilia-Pero/just-screenshotter>

## Описание

Снимает видимую часть страницы или страницу целиком и открывает редактор в новой вкладке. Выделите область, порисуйте на ней карандашом, линией, стрелкой, рамкой, овалом, маркером или напишите текст, затем скопируйте снимок или сохраните его в PNG. Нарисованное можно в любой момент передвинуть, перекрасить, изменить в размере или удалить.

Клавиши: Alt+Shift+S снимает видимую часть, Alt+Shift+P всю страницу. Страница прокручивается и снимается по частям; элементы, закреплённые на экране, остаются только на первой части. В один снимок помещается до 32 000 пикселей страницы по высоте.

Расширение обращается к странице после нажатия его кнопки или клавиш. Снимки остаются на вашем компьютере: сетевых запросов расширение не делает и хранит только выбранный язык. 14 языков. Исходный код открыт, лицензия MIT.

<https://github.com/Ilia-Pero/just-screenshotter>

## Single purpose

Taking a picture of the current page and drawing on it.

## Permissions

| Permission | Purpose |
| --- | --- |
| `activeTab` | to capture the current page after the button or the shortcut is pressed |
| `scripting` | to measure the page, scroll it while the whole page is being captured, and return it to where it was |
| `storage` | to remember the language |
| `clipboardWrite` | to copy the picture when Copy is pressed |

The extension collects no data. All its code is in the package.

## For reviewers

The scripts are bundled from TypeScript by esbuild and are not minified. To build the same files with Node.js 24 on any system:

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run extension
```

The packages are the folders `extension/packed/firefox` and `extension/packed/chrome`. The source is in `extension/src` and `src`.

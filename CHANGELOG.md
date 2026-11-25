# Changes

## 0.5.1 (2026-11-25)

Fixed:

- Closing the settings while they waited for a key left all shortcuts disabled until the program was restarted.
- Opening the settings from the tray did nothing if their window was minimized.
- F5 and Ctrl+R reloaded the editor, and the drawings were lost.
- On keyboards where Z is not in its American place, such as German and French ones, Ctrl+Z did not undo, and a letter chosen for a shortcut could turn into another one.
- A recording that was still being saved could stop the next one.
- On Linux in a language other than English, errors of video recording were not reported. A failed recording left an empty file.
- On macOS and Linux, the key of the server could be read by other users of the computer for an instant while it was being saved.
- On Windows, system audio was opened before a recording even when it was turned off in the settings.
- In the browser extension Ctrl+U showed an error.

Changed:

- The extension builds its icons without `innerHTML`, as Mozilla recommends.
- On X11, repeats of a key that is being held are ignored.

## 0.5.0 (2026-11-25)

- In a text, Enter starts a new line. A click elsewhere, Esc or Ctrl+Enter ends typing.
- The wheel changes the size of letters while a text is being typed.
- While a text is being typed, Ctrl+S and Ctrl+U finish it and then save or upload the area. Ctrl+C and Ctrl+Z do the same if the field is empty.
- The spell checker of the system underlines mistakes in Russian and English, and the context menu offers corrections. On Linux it needs the packages hunspell-ru and hunspell-en-us.
- Ovals. With Shift an oval becomes a circle and a frame becomes a square.
- Turning the wheel over a selected drawing changes only that drawing. In 0.4.0 the tool changed as well.
- “Check for updates” in the settings.
- The extension for Chrome, Edge, Opera and Yandex Browser. Its window shows the shortcuts.
- The settings are divided into sections and fit a narrow window. Contrast, the size of controls and labels for screen readers were improved.

## 0.4.0 (2026-11-25)

- A drawing can be changed at any time: a click selects it with any tool, and a double click edits a text.
- Kazakh and Belarusian, 14 languages in total. The Windows installer does not support Kazakh; choose it in the settings.
- The language moved from the editor to the settings.
- On Linux the editor fits the screen. It used to measure its window before the window was shown, and nothing could be selected.

## 0.3.0 (2026-11-25)

- A window with the settings; the tray menu became shorter.
- Configurable shortcuts for four actions: a screenshot of an area, of the whole screen, of the active window (Windows) and a video.
- On Windows both mouse buttons pressed at once can start an action, alone or with Shift or Ctrl. Off by default.
- The program starts with the system. This can be turned off.
- On Linux the program no longer uses processor time at rest. Before, it checked the keyboard twenty times a second.
- The Firefox extension, which captures the visible part of a page or the whole page.

## 0.2.0 (2026-11-25)

- System audio in video on macOS 13 and later.
- On older macOS recording starts without a delay, and recording with the microphone alone works.

## 0.1.0 (2026-11-25)

- Screenshots of all monitors on Windows, macOS and Linux.
- The editor: selection, pencil, line, arrow, frame, marker, text, eight colors, undo.
- Drawings can be moved, resized, recolored, retyped and deleted.
- Video of an area with system audio and the microphone; on macOS with the microphone only.
- Copying, saving as PNG, uploading to your own server.
- The server for uploaded screenshots.
- 12 languages.

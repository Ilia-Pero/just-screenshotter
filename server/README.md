# The server for screenshots

One file in Python without dependencies, a plain WSGI application. It accepts PNG with a key, keeps it under a random name and shows it by a link. It needs Python 3.12 or newer.

| Variable | What is in it |
| --- | --- |
| `SCREENSHOT_FOLDER` | the folder for screenshots |
| `SCREENSHOT_URL` | the address they are seen at from outside |
| `SCREENSHOT_TOKEN` | the key, at least 32 characters long |
| `SCREENSHOT_PORT` | the port for a try on this computer, 5397 at first |

To try it on this computer:

```sh
python screenshot_server.py
```

On a server any WSGI server runs it behind a web server with https:

```sh
gunicorn "screenshot_server:application()"
```

Then put the same address and key into the program: the menu of the tray icon, "Upload by link…".

Anybody who knows the link of a screenshot can open it. The server does not delete old screenshots by itself.

Tests:

```sh
python -m unittest discover -s tests -t .
```

# Server for screenshots

A WSGI application in one Python file without dependencies. It accepts PNG files with a key, stores them under random names and returns links. Python 3.12 or later is required.

| Variable | Value |
| --- | --- |
| `SCREENSHOT_FOLDER` | the folder for screenshots |
| `SCREENSHOT_URL` | the public address of the server |
| `SCREENSHOT_TOKEN` | the key, at least 32 characters long |
| `SCREENSHOT_PORT` | the port for a local run, 5397 by default |

A local run:

```sh
python screenshot_server.py
```

On a server, use any WSGI server behind a web server with HTTPS:

```sh
gunicorn "screenshot_server:application()"
```

Then open the settings of the program, press “Upload by link…” and enter the same address and key.

Anyone who has the link to a screenshot can open it. The server does not delete old screenshots.

Tests:

```sh
python -m unittest discover -s tests -t .
```

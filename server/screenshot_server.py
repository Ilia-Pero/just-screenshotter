"""A server for screenshots: accepts PNG with a key and shows it by a random link.

This is a plain WSGI application without dependencies. It is set up with environment variables:
SCREENSHOT_FOLDER is the folder for screenshots, SCREENSHOT_URL is the address they are seen at
from outside, SCREENSHOT_TOKEN is the key the program uploads with.

To try it on this computer: python screenshot_server.py
To run it on a server: gunicorn "screenshot_server:application()"
"""

import hmac
import json
import os
import re
import secrets
from collections.abc import Callable, Iterable, Mapping
from dataclasses import dataclass
from http import HTTPStatus
from pathlib import Path
from typing import Any
from wsgiref.simple_server import make_server

SIGNATURE = b"\x89PNG\r\n\x1a\n"
LIMIT = 30 * 1024 * 1024
SHORTEST = 32
NAME = re.compile(r"[A-Za-z0-9_-]{16}\.png")

Environment = Mapping[str, Any]
Start = Callable[[str, list[tuple[str, str]]], object]


@dataclass(frozen=True)
class Answer:
    """An answer of the server."""

    status: HTTPStatus
    kind: str
    body: bytes

    def sent(self, start: Start) -> Iterable[bytes]:
        start(
            f"{self.status.value} {self.status.phrase}",
            [
                ("Content-Type", self.kind),
                ("Content-Length", str(len(self.body))),
                ("X-Content-Type-Options", "nosniff"),
                ("X-Robots-Tag", "noindex"),
                ("Referrer-Policy", "no-referrer"),
            ],
        )
        return [self.body]


def refusal(status: HTTPStatus, text: str) -> Answer:
    return Answer(status, "text/plain; charset=utf-8", text.encode())


@dataclass(frozen=True)
class Shots:
    """A folder with screenshots."""

    folder: Path

    def stored(self, png: bytes) -> str:
        """Writes the screenshot under a new random name and returns the name."""
        self.folder.mkdir(parents=True, exist_ok=True)
        name = f"{secrets.token_urlsafe(12)}.png"
        with (self.folder / name).open("xb") as file:
            file.write(png)
        return name

    def found(self, name: str) -> tuple[bytes, ...]:
        """The screenshot with the name. Empty if there is none or the name is not ours."""
        if NAME.fullmatch(name) is None:
            return ()
        try:
            return ((self.folder / name).read_bytes(),)
        except FileNotFoundError:
            return ()


@dataclass(frozen=True)
class Site:
    """A site with screenshots: POST accepts them, GET shows them."""

    shots: Shots
    url: str
    token: str

    def __call__(self, environment: Environment, start: Start) -> Iterable[bytes]:
        method = environment.get("REQUEST_METHOD", "GET")
        if method == "POST":
            return self.accepted(environment).sent(start)
        if method in ("GET", "HEAD"):
            sent = self.shown(str(environment.get("PATH_INFO", "/"))).sent(start)
            return sent if method == "GET" else []
        return refusal(HTTPStatus.METHOD_NOT_ALLOWED, "Not allowed").sent(start)

    def accepted(self, environment: Environment) -> Answer:
        given = str(environment.get("HTTP_AUTHORIZATION", "")).removeprefix("Bearer ")
        if not hmac.compare_digest(given.encode(), self.token.encode()):
            return refusal(HTTPStatus.UNAUTHORIZED, "The key does not fit")
        length = str(environment.get("CONTENT_LENGTH", ""))
        if not length.isdecimal():
            return refusal(HTTPStatus.LENGTH_REQUIRED, "The size is not given")
        if int(length) > LIMIT:
            return refusal(HTTPStatus.REQUEST_ENTITY_TOO_LARGE, "The screenshot is too large")
        png = environment["wsgi.input"].read(int(length))
        if len(png) != int(length) or not png.startswith(SIGNATURE):
            return refusal(HTTPStatus.UNSUPPORTED_MEDIA_TYPE, "This is not a PNG screenshot")
        link = f"{self.url.rstrip('/')}/{self.shots.stored(png)}"
        return Answer(
            HTTPStatus.CREATED, "application/json", json.dumps({"url": link}).encode()
        )

    def shown(self, path: str) -> Answer:
        for png in self.shots.found(path.rsplit("/", 1)[-1]):
            return Answer(HTTPStatus.OK, "image/png", png)
        return refusal(HTTPStatus.NOT_FOUND, "There is no such screenshot")


def application() -> Site:
    """The site described by the environment variables."""
    token = os.environ["SCREENSHOT_TOKEN"]
    if len(token) < SHORTEST:
        raise ValueError(f"SCREENSHOT_TOKEN is shorter than {SHORTEST} characters")
    return Site(
        Shots(Path(os.environ["SCREENSHOT_FOLDER"])), os.environ["SCREENSHOT_URL"], token
    )


if __name__ == "__main__":
    port = int(os.environ.get("SCREENSHOT_PORT", "5397"))
    with make_server("127.0.0.1", port, application()) as server:
        server.serve_forever()

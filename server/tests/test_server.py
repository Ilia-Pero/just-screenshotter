import io
import json
import tempfile
import unittest
from pathlib import Path

from screenshot_server import LIMIT, SIGNATURE, Shots, Site

TOKEN = "t" * 32
PNG = SIGNATURE + b"body"


class Visit:
    """One request to the site: what it answers and what it gives."""

    def __init__(self, folder: str, **environment: object) -> None:
        self.site = Site(Shots(Path(folder)), "https://shots.example/s", TOKEN)
        self.environment = environment
        self.heard: list[str] = []

    def body(self) -> bytes:
        return b"".join(self.site(self.environment, lambda status, _: self.heard.append(status)))

    def status(self) -> str:
        self.body()
        return self.heard[0]


def upload(folder: str, png: bytes = PNG, token: str = TOKEN, length: str = "") -> Visit:
    return Visit(
        folder,
        REQUEST_METHOD="POST",
        HTTP_AUTHORIZATION=f"Bearer {token}",
        CONTENT_LENGTH=length or str(len(png)),
        **{"wsgi.input": io.BytesIO(png)},
    )


def page(folder: str, path: str, method: str = "GET") -> Visit:
    return Visit(folder, REQUEST_METHOD=method, PATH_INFO=path)


class SiteTest(unittest.TestCase):
    def setUp(self) -> None:
        holder = tempfile.TemporaryDirectory()
        self.addCleanup(holder.cleanup)
        self.folder = holder.name

    def link(self) -> str:
        return json.loads(upload(self.folder).body())["url"]

    def test_link_leads_to_public_address(self) -> None:
        self.assertTrue(self.link().startswith("https://shots.example/s/"))

    def test_shot_opens_by_its_link(self) -> None:
        self.assertEqual(page(self.folder, "/s/" + self.link().rsplit("/", 1)[-1]).body(), PNG)

    def test_every_shot_gets_its_own_link(self) -> None:
        self.assertNotEqual(self.link(), self.link())

    def test_wrong_key_is_refused(self) -> None:
        self.assertTrue(upload(self.folder, token="x" * 32).status().startswith("401"))

    def test_missing_key_is_refused(self) -> None:
        self.assertTrue(upload(self.folder, token="").status().startswith("401"))

    def test_nothing_is_stored_with_wrong_key(self) -> None:
        upload(self.folder, token="x" * 32).body()
        self.assertEqual(list(Path(self.folder).iterdir()), [])

    def test_other_files_are_refused(self) -> None:
        self.assertTrue(upload(self.folder, png=b"<html>").status().startswith("415"))

    def test_huge_shot_is_refused_before_reading(self) -> None:
        self.assertTrue(upload(self.folder, length=str(LIMIT + 1)).status().startswith("413"))

    def test_unknown_size_is_refused(self) -> None:
        self.assertTrue(upload(self.folder, length="many").status().startswith("411"))

    def test_torn_shot_is_refused(self) -> None:
        self.assertTrue(upload(self.folder, length=str(len(PNG) + 5)).status().startswith("415"))

    def test_path_outside_folder_is_not_found(self) -> None:
        self.assertTrue(page(self.folder, "/s/..%2f..%2fsecret.png").status().startswith("404"))

    def test_unknown_shot_is_not_found(self) -> None:
        self.assertTrue(page(self.folder, "/s/aaaaaaaaaaaaaaaa.png").status().startswith("404"))

    def test_head_has_no_body(self) -> None:
        name = self.link().rsplit("/", 1)[-1]
        self.assertEqual(page(self.folder, f"/s/{name}", method="HEAD").body(), b"")

    def test_other_methods_are_refused(self) -> None:
        self.assertTrue(page(self.folder, "/", method="DELETE").status().startswith("405"))


if __name__ == "__main__":
    unittest.main()

<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Windows, macOS, Linux용 스크린샷과 화면 녹화</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · **한국어** · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![예시 화면 위의 편집기: 형광펜, 타원, 화살표, 글자](editor.png)

Print Screen을 누르면 모든 모니터를 캡처하고 그 위에 편집기를 엽니다. 영역을 선택해 그린 뒤 이미지를 복사하거나 저장하거나 업로드합니다. 같은 영역을 동영상으로 녹화할 수도 있습니다. 캡처하지 않을 때는 트레이 아이콘이 있는 프로세스 하나만 메모리에 남습니다.

## 설치

| 시스템 | 파일 | 설치 방법 |
| --- | --- | --- |
| Windows 10, 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | 파일을 실행합니다. Windows 경고가 나오면 “추가 정보”, “실행”을 차례로 선택합니다. |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | 프로그램을 응용 프로그램 폴더로 옮긴 뒤 엽니다. macOS가 처음 거부하면 시스템 설정의 “개인정보 보호 및 보안”에서 “그래도 열기”를 누릅니다. |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | `about:debugging`을 열고 “이 Firefox”, “임시 부가 기능 로드”에서 파일을 선택합니다. Firefox를 다시 시작하면 다시 로드해야 합니다. |
| Chrome, Edge, Opera, Yandex Browser | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | 지우지 않을 폴더에 압축을 풉니다. `chrome://extensions`를 열고 개발자 모드를 켠 뒤 “압축해제된 확장 프로그램을 로드합니다”를 눌러 폴더를 선택합니다. |

파일에 유료 서명이 없어 Windows와 macOS가 경고를 표시합니다. GitHub가 이 저장소에서 파일을 만들고, 깨끗한 컴퓨터에 프로그램을 설치하고 실행한 뒤에 공개합니다.

## 기능

- 연필, 선, 화살표, 틀, 타원, 형광펜, 글자로 그리기: 8가지 색, 굵기 1~72픽셀.
- 그린 것은 언제든지 옮기고, 색과 크기를 바꾸고, 글자를 다시 쓰고, 지울 수 있습니다.
- 이미지를 복사하거나 PNG로 저장하거나 [자신의 서버](../server/README.md)에 업로드합니다.
- 영역을 컴퓨터 소리와 마이크와 함께 녹화: Windows는 MP4, macOS는 MOV, Linux는 WebM. macOS 13 이전에는 마이크만 녹음합니다.
- 14개 언어. 키와 폴더는 설정에서 정합니다.

## 키

| 동작 | Windows, Linux | macOS |
| --- | --- | --- |
| 영역 스크린샷 | Print Screen | ⌘⇧9 |
| 전체 화면 스크린샷 | Shift + Print Screen | |
| 활성 창 스크린샷, Windows 전용 | Alt + Print Screen | |
| 영역 동영상 | Ctrl + Shift + Print Screen | ⌘⇧8 |

키는 설정에서 바꿀 수 있습니다. 편집기에서 Ctrl+C는 복사, Ctrl+S는 저장, Ctrl+Z는 실행 취소, Esc는 닫기입니다.

## 브라우저 확장 프로그램

페이지의 보이는 부분(Alt+Shift+S)이나 페이지 전체(Alt+Shift+P)를 캡처하고 새 탭에서 같은 편집기를 엽니다. Firefox와 Chromium 기반 브라우저에서 동작합니다.

## 데이터와 네트워크

이미지와 녹화는 사용자의 컴퓨터에 남습니다. 자신의 서버에 이미지를 올릴 때와 “업데이트 확인”을 누를 때 네트워크를 사용합니다. 계정과 원격 측정은 없습니다.

## 더 보기

측정 결과, 확인한 시스템, 빌드 방법은 [영어 문서](../README.md)에 있습니다.

이 글은 기계 번역이며 원어민의 검토를 받지 않았습니다. 수정은 `docs` 폴더에 해 주세요.

[MIT](../LICENSE)

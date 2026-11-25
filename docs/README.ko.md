<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">대기 중 부하가 없는 스크린샷 도구 (Windows, macOS, Linux)</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · **한국어** · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![예시 화면 위의 편집기: 형광펜, 타원, 화살표, 글자](editor.png)

Print Screen을 누르고 영역을 고른 뒤 그 위에 그리고, 그림을 복사하거나 저장하거나 업로드합니다. 영역을 동영상으로 녹화할 수도 있습니다. 촬영 사이에는 키를 기다리기만 합니다. 창도 타이머도 없으며 프로세서를 쓰지 않습니다.

## 하는 일

- 모든 모니터를 찍고 그 위에 편집기를 엽니다.
- 연필, 직선, 화살표, 테두리, 타원, 형광펜, 글자를 여덟 가지 색으로 그립니다.
- 이미 그린 것을 바꿀 수 있습니다. 클릭해서 고른 뒤 전체나 끝점을 끌고, 굵기와 색을 바꾸고, 글자를 다시 입력하거나 삭제합니다.
- 그림을 복사하거나 PNG로 저장하거나 [자신의 서버](../server/README.md)에 업로드하고 링크를 복사합니다.
- 영역을 소리가 있는 동영상으로 녹화합니다. Windows는 MP4, macOS는 MOV, Linux는 WebM입니다. 컴퓨터 소리와 마이크를 녹음할지는 직접 고릅니다. macOS 13보다 이전에서는 마이크만 됩니다. 중지 버튼, 단축키, 트레이 아이콘으로 녹화를 끝냅니다.
- 14개 언어를 지원합니다. 언어는 설정과 Windows 설치 프로그램에서 고릅니다. 그전에는 시스템 언어를 씁니다.
- 스스로 아무것도 보내지 않습니다. 계정과 원격 측정이 없습니다. 지정한 서버에 그림을 올릴 때와 설정의 단추로 새 버전을 확인할 때만 네트워크를 씁니다.

## 키와 마우스

| 하는 일 | Windows와 Linux | macOS |
| --- | --- | --- |
| 영역 스크린샷 | Print Screen | ⌘⇧9 |
| 전체 화면 스크린샷 | Shift + Print Screen | — |
| 활성 창 스크린샷 (Windows) | Alt + Print Screen | — |
| 영역 동영상 | Ctrl + Shift + Print Screen | ⌘⇧8 |

다른 키는 설정에서 고릅니다. Windows에서는 마우스 버튼으로도 같은 일을 시작할 수 있습니다. 양쪽 버튼을 함께, 그냥 또는 Shift나 Ctrl과 같이 누릅니다. 처음에는 꺼져 있습니다. 버튼을 지켜보는 동안에는 마우스를 움직일 때마다 프로세서 시간을 약 2µs 쓰기 때문입니다. 가만히 있을 때는 쓰지 않습니다. Wayland에서는 시스템 설정에서 키를 지정합니다.

## 편집기의 키

| 키 | 하는 일 |
| --- | --- |
| Ctrl+C | 영역 복사 |
| Ctrl+S | 영역을 `사진/Screenshots`에 저장 |
| Ctrl+U | 영역을 업로드하고 링크를 클립보드로 |
| Ctrl+Z | 마지막 그림 취소 |
| Delete | 고른 표시 삭제 |
| Esc | 편집기 닫기 |

macOS에서는 Ctrl 대신 ⌘을 씁니다.

어떤 도구를 쓰든 클릭하면 표시를 고릅니다. 두 번 클릭하거나 Enter를 누르면 글자를 바꾸고, 휠로 굵기를 바꿉니다.

글자를 쓸 때 Enter는 새 줄을 시작하고, 다른 곳을 클릭하거나 Esc를 누르면 끝납니다. 휠은 글자 크기를 바꿉니다. 러시아어와 영어의 틀린 곳에는 시스템이 밑줄을 긋고, 오른쪽 단추로 고칠 말을 고릅니다. Shift를 누르면 테두리는 정사각형, 타원은 원이 됩니다.

나머지도 설정에 있습니다. 동영상과 함께 녹음할 것, 폴더, 언어, 업로드, 시스템과 함께 시작입니다. 시스템과 함께 시작은 끌 때까지 켜져 있습니다. 설정은 트레이 아이콘 메뉴에서 열거나 프로그램을 `--settings`로 실행해 엽니다. 그것 없이 다시 실행하면 스크린샷을 찍습니다.

## 브라우저 확장 기능

Firefox와 Chromium 기반 브라우저(Chrome, Yandex 브라우저, Edge, Opera)에서 씁니다.

페이지의 보이는 부분이나 전체 페이지를 찍고 같은 편집기를 새 탭에서 엽니다. 선택하고, 그리고, 복사하고, 저장합니다. 키는 Alt+Shift+S와 Alt+Shift+P입니다. 요청할 때만 페이지를 보고, 아무것도 보내지 않으며, 언어만 기억합니다. 동영상은 녹화하지 않고 그림 높이는 32,000픽셀까지입니다.

## 얼마나 가벼운가

2026-11-24, Windows Server 2022, 1920 × 1080 화면, 릴리스 빌드에서 측정:

| 항목 | 결과 |
| --- | --- |
| 대기 중 프로세서 시간 | 10초 동안 0ms |
| 대기 중 메모리 | 첫 촬영 전 14MB, 일곱 번 뒤 25MB |
| 대기 중 프로세스 수 | 1 |
| 키를 누른 뒤 편집기가 뜰 때까지 | 일곱 번 측정 0.61~0.75초 |
| 프로그램 크기 | 5.2MB |

편집기가 열리는 데 이만큼 걸리는 것은 촬영할 때마다 창을 만들고 끝나면 없애기 때문입니다. 대기 중 부하를 없앤 대가입니다.

동영상은 별도 프로세스가 녹화하며, 녹화가 끝나면 메모리를 모두 돌려주고 사라집니다. 그래픽 카드가 없는 이 서버에서 1920 × 1080을 초당 30프레임으로 녹화하면 코어 하나와 207MB, 800 × 600은 코어 0.4개와 78MB를 씁니다. H.264를 인코딩하는 그래픽 카드가 이 일을 맡으면 달라지지만 아직 재지 않았습니다.

## 상태

| 시스템 | 상태 |
| --- | --- |
| Windows | Windows Server 2022와 모니터 한 대에서 동작합니다. 동영상도 거기서 확인했습니다. 서버에 소리 장치가 없어 장치 녹음은 확인하지 못했습니다. Windows 10과 11에서는 아직 확인하지 않았습니다. |
| macOS 10.13+ | 빌드되고 테스트를 통과하며, GitHub의 컴퓨터에서 설치, 실행, 컴퓨터 소리를 포함한 화면 녹화가 됩니다. 아직 아무도 직접 써 보지 않았습니다. |
| Ubuntu 24.04+ | 빌드되고 테스트를 통과하며, GitHub의 컴퓨터에서 설치, 실행, 화면 녹화가 됩니다. Ubuntu 26.04의 가상 X11 화면에서 단축키, 영역 선택, 저장, 동영상이 동작합니다. Wayland에서는 확인하지 않았습니다. 아직 아무도 직접 써 보지 않았습니다. Wayland에서는 키보드 설정에서 단축키를 지정하세요. 명령은 `just-screenshotter`입니다. |
| Firefox 128+ | Firefox 161에서 동작합니다(창 없이 확인). 아직 Mozilla의 서명이 없어 Firefox를 다시 시작하면 다시 로드해야 합니다. |
| Chrome, Yandex 브라우저, Edge, Opera | Edge 158에서 동작합니다(창 없이 확인). 다른 브라우저는 확인하지 않았으며 Chromium 120 이상이 필요합니다. |

## 설치

| 시스템 | 파일 | 설치 방법 |
| --- | --- | --- |
| Windows 10, 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | 실행합니다. Windows가 경고하면 "추가 정보", "실행"을 고릅니다. |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | 프로그램을 응용 프로그램으로 끌어다 놓고 엽니다. macOS가 거부하면 시스템 설정의 "개인정보 보호 및 보안"에서 "그래도 열기"를 누릅니다. |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | `about:debugging`을 열어 "현재 Firefox", "임시 부가 기능 로드", 이 파일을 고릅니다. Firefox를 다시 시작하면 다시 로드합니다. |
| Chrome, Yandex 브라우저, Edge, Opera | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | 압축을 풀고 `chrome://extensions`를 열어 "개발자 모드"를 켠 다음 "압축해제된 확장 프로그램 로드"로 폴더를 고릅니다. 폴더는 지우지 마세요. Yandex 브라우저는 스토어만 믿으므로 확장 기능을 끌 수 있습니다. |

GitHub가 이 저장소의 코드로 파일을 빌드하고 하나씩 설치해 프로그램을 실행해 본 뒤에 공개합니다. 유료 서명이 없어 Windows와 macOS가 첫 실행 때 경고합니다.

직접 빌드할 수도 있습니다. Node.js 24 이상과 Rust가 필요하고, Windows에서는 Microsoft C++ Build Tools, Ubuntu에서는 `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`도 필요합니다.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

설치 파일은 `src-tauri/target/release/bundle`에 생깁니다.

## 번역

번역은 기계 번역이며 원어민이 확인하지 않았습니다. 잘못된 곳을 보면 `locales`나 `docs`에서 고쳐 주세요.

## 참여와 라이선스

빌드, 테스트, 수정 방법: [CONTRIBUTING.md](../CONTRIBUTING.md). 라이선스는 [MIT](../LICENSE)입니다.

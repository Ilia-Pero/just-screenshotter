<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">대기 중 부하가 없는 스크린샷 도구 (Windows, macOS, Linux)</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · **한국어** · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md)

![예시 화면 위의 편집기: 형광펜, 테두리, 화살표, 글자](editor.png)

Print Screen을 누르고 영역을 고른 뒤 그 위에 그리고, 그림을 복사하거나 저장하거나 업로드합니다. 촬영 사이에는 키를 기다리기만 합니다. 창도 타이머도 없으며 프로세서를 쓰지 않습니다.

## 하는 일

- 모든 모니터를 찍고 그 위에 편집기를 엽니다.
- 연필, 직선, 화살표, 테두리, 형광펜, 글자를 여덟 가지 색으로 그립니다.
- 그림을 복사하거나 PNG로 저장하거나 [자신의 서버](../server/README.md)에 업로드하고 링크를 복사합니다.
- 12개 언어를 지원합니다. 처음에는 시스템 언어를 쓰며 메뉴에서 바꿀 수 있습니다.
- 스스로 아무것도 보내지 않습니다. 계정, 원격 측정, 업데이트 확인이 없습니다. 지정한 서버에 그림을 올릴 때만 네트워크를 씁니다.

## 편집기의 키

| 키 | 하는 일 |
| --- | --- |
| Ctrl+C | 영역 복사 |
| Ctrl+S | 영역을 `사진/Screenshots`에 저장 |
| Ctrl+U | 영역을 업로드하고 링크를 클립보드로 |
| Ctrl+Z | 마지막 그림 취소 |
| Esc | 편집기 닫기 |

macOS에서는 Ctrl 대신 ⌘을 씁니다.

트레이 아이콘 메뉴에 단축키, 폴더, 언어, 서버 주소, 시스템과 함께 시작이 있습니다. 프로그램을 다시 실행해도 스크린샷을 찍습니다.

## 얼마나 가벼운가

2026-11-24, Windows Server 2022, 1920 × 1080 화면, 릴리스 빌드에서 측정:

| 항목 | 결과 |
| --- | --- |
| 대기 중 프로세서 시간 | 10초 동안 0ms |
| 대기 중 메모리 | 첫 촬영 전 14MB, 일곱 번 뒤 25MB |
| 대기 중 프로세스 수 | 1 |
| 키를 누른 뒤 편집기가 뜰 때까지 | 일곱 번 측정 0.61~0.75초 |
| 프로그램 크기 | 5.0MB |

편집기가 열리는 데 이만큼 걸리는 것은 촬영할 때마다 창을 만들고 끝나면 없애기 때문입니다. 대기 중 부하를 없앤 대가입니다.

## 상태

| 시스템 | 상태 |
| --- | --- |
| Windows | Windows Server 2022와 모니터 한 대에서 동작합니다. Windows 10과 11에서는 아직 확인하지 않았습니다. |
| macOS 10.13+ | 빌드되고 테스트를 통과합니다. 아직 아무도 실행해 보지 않았습니다. |
| Ubuntu 24.04+ | 빌드되고 테스트를 통과합니다. 아직 아무도 실행해 보지 않았습니다. Wayland에서는 키보드 설정에서 단축키를 지정하세요. 명령은 `just-screenshotter`입니다. |

## 설치

아직 내려받을 수 있는 파일이 없어 직접 빌드해야 합니다. Node.js 24 이상과 Rust가 필요하고, Windows에서는 Microsoft C++ Build Tools, Ubuntu에서는 `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`도 필요합니다.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

설치 파일은 `src-tauri/target/release/bundle`에 생깁니다. 서명되지 않았으므로 Windows와 macOS가 신뢰할지 묻습니다.

## 번역

번역은 기계 번역이며 원어민이 확인하지 않았습니다. 잘못된 곳을 보면 `locales`나 `docs`에서 고쳐 주세요.

## 참여와 라이선스

빌드, 테스트, 수정 방법: [CONTRIBUTING.md](../CONTRIBUTING.md). 라이선스는 [MIT](../LICENSE)입니다.

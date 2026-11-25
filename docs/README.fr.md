<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Captures et enregistrement d’écran pour Windows, macOS et Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · **Français** · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![L’éditeur sur un écran d’exemple : marqueur, ovale, flèche et texte](editor.png)

Avec la touche Print Screen, le programme capture tous les écrans et ouvre un éditeur par-dessus. On y sélectionne une zone, on dessine dessus, puis on copie, enregistre ou envoie l’image. La même zone peut être enregistrée en vidéo. Entre deux captures, un seul processus reste en mémoire, avec une icône dans la zone de notification.

## Installation

| Système | Fichier | Comment installer |
| --- | --- | --- |
| Windows 10, 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Lancez le fichier. Si Windows affiche un avertissement, choisissez « Informations complémentaires », puis « Exécuter quand même ». |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Glissez le programme dans Applications et ouvrez-le. Après le premier refus de macOS, ouvrez Réglages Système, « Confidentialité et sécurité », et cliquez sur « Ouvrir quand même ». |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | Ouvrez `about:debugging`, « Ce Firefox », « Charger un module complémentaire temporaire » et choisissez le fichier. Après un redémarrage de Firefox, chargez-le à nouveau. |
| Chrome, Edge, Opera, Yandex Browser | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | Décompressez l’archive dans un dossier que vous garderez. Ouvrez `chrome://extensions`, activez le mode développeur, cliquez sur « Charger l’extension non empaquetée » et choisissez le dossier. |

Windows et macOS affichent un avertissement parce que les fichiers n’ont pas de signature payante. GitHub les compile à partir de ce dépôt, installe les programmes sur des machines vierges et les lance avant de publier une version.

## Fonctions

- Dessin au crayon, ligne, flèche, cadre, ovale, marqueur et texte : huit couleurs, épaisseur de 1 à 72 pixels.
- Un dessin peut être déplacé, recoloré, redimensionné, réécrit ou supprimé à tout moment.
- L’image est copiée, enregistrée en PNG ou envoyée sur [votre serveur](../server/README.md).
- Vidéo d’une zone avec le son de l’ordinateur et le microphone : MP4 sous Windows, MOV sous macOS, WebM sous Linux. Avant macOS 13, seul le microphone est enregistré.
- 14 langues ; les touches et le dossier se choisissent dans les réglages.

## Touches

| Action | Windows, Linux | macOS |
| --- | --- | --- |
| Capture d’une zone | Print Screen | ⌘⇧9 |
| Capture de tout l’écran | Shift + Print Screen | |
| Capture de la fenêtre active, Windows seulement | Alt + Print Screen | |
| Vidéo d’une zone | Ctrl + Shift + Print Screen | ⌘⇧8 |

Les touches se changent dans les réglages. Dans l’éditeur, Ctrl+C copie, Ctrl+S enregistre, Ctrl+Z annule et Échap ferme.

## Extension de navigateur

Capture la partie visible d’une page (Alt+Shift+S) ou la page entière (Alt+Shift+P) et ouvre le même éditeur dans un nouvel onglet. Fonctionne dans Firefox et dans les navigateurs basés sur Chromium.

## Données et réseau

Les captures et les vidéos restent sur votre ordinateur. Le programme utilise le réseau quand vous envoyez une capture sur votre serveur et quand vous cliquez sur « Rechercher des mises à jour ». Il n’a ni compte ni télémétrie.

## En savoir plus

Les mesures, les systèmes vérifiés et la compilation sont décrits [en anglais](../README.md).

Ce texte a été traduit par une machine et n’a pas été relu par des locuteurs natifs. Les corrections se font dans le dossier `docs`.

[MIT](../LICENSE)

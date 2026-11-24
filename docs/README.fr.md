<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Un outil de capture d’écran sans charge au repos pour Windows, macOS et Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · **Français** · [Deutsch](README.de.md) · [Italiano](README.it.md)

![L’éditeur au-dessus d’un écran d’exemple : un surligneur, un cadre, une flèche et un texte](editor.png)

Appuyez sur Print Screen, sélectionnez une zone, dessinez dessus, puis copiez, enregistrez ou envoyez l’image. Entre deux captures, le programme attend seulement la touche : il n’a ni fenêtre ni minuteur et n’occupe pas le processeur.

## Ce qu’il fait

- Capture tous les écrans et ouvre l’éditeur par-dessus.
- Dessine au crayon, avec une ligne, une flèche, un cadre, un surligneur et du texte, en huit couleurs.
- Copie l’image, l’enregistre en PNG ou l’envoie sur [votre propre serveur](../server/README.md) et copie le lien.
- Parle 12 langues. Il utilise d’abord celle du système ; une autre se choisit dans le menu.
- N’envoie rien de lui-même : pas de compte, pas de télémétrie, pas de recherche de mises à jour. Il n’utilise le réseau que pour envoyer une image au serveur que vous avez indiqué.

## Touches dans l’éditeur

| Touche | Ce qu’elle fait |
| --- | --- |
| Ctrl+C | copie la zone |
| Ctrl+S | enregistre la zone dans `Images/Screenshots` |
| Ctrl+U | envoie la zone, le lien va dans le presse-papiers |
| Ctrl+Z | annule le dernier dessin |
| Esc | ferme l’éditeur |

Sous macOS, ⌘ remplace Ctrl.

Le menu de l’icône contient le raccourci, le dossier, la langue, l’adresse du serveur et le démarrage avec le système. Lancer le programme une seconde fois prend aussi une capture.

## Sa légèreté

Mesuré le 24.11.2026 sous Windows Server 2022, écran 1920 × 1080, version optimisée :

| Mesure | Résultat |
| --- | --- |
| Processeur au repos | 0 ms en 10 s |
| Mémoire au repos | 14 Mo avant la première capture, 25 Mo après sept |
| Processus au repos | 1 |
| De la touche à l’éditeur | 0,61–0,75 s sur sept essais |
| Taille du programme | 5,0 Mo |

L’éditeur met ce temps à s’ouvrir parce que sa fenêtre est créée à chaque capture puis détruite. C’est le prix d’une charge nulle au repos.

## État

| Système | État |
| --- | --- |
| Windows | Fonctionne sous Windows Server 2022 avec un écran. Pas encore vérifié sous Windows 10 et 11. |
| macOS 10.13+ | Se compile et passe les tests. Personne ne l’a encore lancé. |
| Ubuntu 24.04+ | Se compile et passe les tests. Personne ne l’a encore lancé. Sous Wayland, attribuez le raccourci dans les réglages du clavier : la commande est `just-screenshotter`. |

## Installation

Il n’y a pas encore de fichiers prêts à télécharger, il faut donc compiler le programme. Il faut Node.js 24 ou plus récent et Rust, sous Windows aussi Microsoft C++ Build Tools, sous Ubuntu aussi `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

L’installateur apparaît dans `src-tauri/target/release/bundle`. Il n’est pas signé, Windows et macOS demanderont donc si vous lui faites confiance.

## Traductions

Les traductions sont automatiques et n’ont pas été relues par des locuteurs natifs. Si vous voyez une erreur, corrigez-la dans `locales` ou dans `docs`.

## Participer et licence

Comment compiler, tester et modifier le programme : [CONTRIBUTING.md](../CONTRIBUTING.md). La licence est [MIT](../LICENSE).

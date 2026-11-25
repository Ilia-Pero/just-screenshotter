<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Un outil de capture d’écran sans charge au repos pour Windows, macOS et Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · **Français** · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![L’éditeur au-dessus d’un écran d’exemple : un surligneur, un cadre, une flèche et un texte](editor.png)

Appuyez sur Print Screen, sélectionnez une zone, dessinez dessus, puis copiez, enregistrez ou envoyez l’image, ou enregistrez la zone en vidéo. Entre deux captures, le programme attend seulement la touche : il n’a ni fenêtre ni minuteur et n’occupe pas le processeur.

## Ce qu’il fait

- Capture tous les écrans et ouvre l’éditeur par-dessus.
- Dessine au crayon, avec une ligne, une flèche, un cadre, un ovale, un surligneur et du texte, en huit couleurs.
- Modifie ce qui est déjà dessiné : un clic choisit une marque, puis on la déplace ou on tire ses extrémités, on change son épaisseur et sa couleur, on réécrit son texte ou on la supprime.
- Copie l’image, l’enregistre en PNG ou l’envoie sur [votre propre serveur](../server/README.md) et copie le lien.
- Enregistre la zone en vidéo avec le son : MP4 sous Windows, MOV sous macOS, WebM sous Linux. Vous choisissez d’enregistrer ou non le son de l’ordinateur et le microphone ; sous macOS antérieur à 13, seulement le microphone. Le bouton Arrêter, le raccourci ou l’icône termine l’enregistrement.
- Parle 14 langues. La langue se choisit dans les réglages et dans l’installateur pour Windows ; jusque-là, c’est celle du système.
- N’envoie rien de lui-même : pas de compte, pas de télémétrie. Il n’utilise le réseau que pour envoyer une image au serveur que vous avez indiqué et pour rechercher une nouvelle version quand vous appuyez sur le bouton des réglages.

## Touches et souris

| Ce qu’elles font | Windows et Linux | macOS |
| --- | --- | --- |
| Capture d’une zone | Print Screen | ⌘⇧9 |
| Capture de tout l’écran | Shift + Print Screen | — |
| Capture de la fenêtre active (Windows) | Alt + Print Screen | — |
| Vidéo d’une zone | Ctrl + Shift + Print Screen | ⌘⇧8 |

D’autres touches se choisissent dans les réglages. Sous Windows, les boutons de la souris lancent la même chose : les deux à la fois, seuls, avec Maj ou avec Ctrl. C’est désactivé au départ, car tant que le programme surveille les boutons, chaque mouvement de la souris prend environ 2 µs de processeur ; au repos, rien. Sous Wayland, la touche s’attribue dans les réglages du système.

## Touches dans l’éditeur

| Touche | Ce qu’elle fait |
| --- | --- |
| Ctrl+C | copie la zone |
| Ctrl+S | enregistre la zone dans `Images/Screenshots` |
| Ctrl+U | envoie la zone, le lien va dans le presse-papiers |
| Ctrl+Z | annule le dernier dessin |
| Delete | supprime la marque choisie |
| Esc | ferme l’éditeur |

Sous macOS, ⌘ remplace Ctrl.

Un clic choisit une marque avec n’importe quel outil. Un double clic ou Entrée modifie un texte. La molette change l’épaisseur.

Dans un texte, Entrée commence une nouvelle ligne ; un clic ailleurs ou Échap termine le texte. La molette change la taille des lettres. Le système souligne les fautes en russe et en anglais, et le bouton droit de la souris propose des corrections. Avec Maj, le cadre devient un carré et l’ovale un cercle.

Le reste est aussi dans les réglages : ce qui est enregistré avec la vidéo, le dossier, la langue, l’envoi et le démarrage avec le système, activé jusqu’à ce que vous le désactiviez. Les réglages s’ouvrent depuis le menu de l’icône ou en lançant le programme avec `--settings`. Le lancer une seconde fois sans cela prend aussi une capture.

## L’extension pour le navigateur

Pour Firefox et pour les navigateurs basés sur Chromium : Chrome, Yandex Browser, Edge, Opera.

Capture la partie visible d’une page ou la page entière et ouvre le même éditeur dans un nouvel onglet : sélectionner, dessiner, copier, enregistrer. Les touches sont Alt+Maj+S et Alt+Maj+P. Elle ne regarde une page que si vous le demandez, n’envoie rien et ne retient que la langue. Elle n’enregistre pas de vidéo, et une capture ne dépasse pas 32 000 pixels de haut.

## Sa légèreté

Mesuré le 24.11.2026 sous Windows Server 2022, écran 1920 × 1080, version optimisée :

| Mesure | Résultat |
| --- | --- |
| Processeur au repos | 0 ms en 10 s |
| Mémoire au repos | 14 Mo avant la première capture, 25 Mo après sept |
| Processus au repos | 1 |
| De la touche à l’éditeur | 0,61–0,75 s sur sept essais |
| Taille du programme | 5,2 Mo |

L’éditeur met ce temps à s’ouvrir parce que sa fenêtre est créée à chaque capture puis détruite. C’est le prix d’une charge nulle au repos.

La vidéo est enregistrée par un processus à part, qui disparaît avec toute sa mémoire à la fin. Sur ce serveur sans carte graphique, 30 images par seconde en 1920 × 1080 occupent un cœur du processeur et 207 Mo ; en 800 × 600, 0,4 cœur et 78 Mo. Une carte graphique qui encode le H.264 prend ce travail en charge ; cela n’a pas encore été mesuré.

## État

| Système | État |
| --- | --- |
| Windows | Fonctionne sous Windows Server 2022 avec un écran. La vidéo y a été vérifiée aussi ; l’enregistrement du son des périphériques non, car le serveur n’en a pas. Pas encore vérifié sous Windows 10 et 11. |
| macOS 10.13+ | Se compile et passe les tests ; sur une machine de GitHub, il s’installe, démarre et enregistre l’écran avec le son de l’ordinateur. Personne ne l’a encore essayé à la main. |
| Ubuntu 24.04+ | Se compile et passe les tests ; sur une machine de GitHub, il s’installe, démarre et enregistre l’écran. Sur un écran X11 virtuel d’Ubuntu 26.04, les touches, la sélection, l’enregistrement et la vidéo fonctionnent ; sous Wayland, cela n’a pas été vérifié. Personne ne l’a encore essayé à la main. Sous Wayland, attribuez le raccourci dans les réglages du clavier : la commande est `just-screenshotter`. |
| Firefox 128+ | Fonctionne dans Firefox 161, vérifiée sans fenêtre. Pas encore signée par Mozilla, il faut donc la recharger après un redémarrage de Firefox. |
| Chrome, Yandex Browser, Edge, Opera | Fonctionne dans Edge 158, vérifiée sans fenêtre. Les autres n’ont pas été vérifiés ; il faut Chromium 120 ou plus récent. |

## Installation

| Système | Fichier | Comment l’installer |
| --- | --- | --- |
| Windows 10 et 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Lancez-le. Si Windows ne connaît pas le programme : « Informations complémentaires », « Exécuter quand même ». |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Glissez le programme dans Applications et ouvrez-le. Quand macOS refuse, allez dans Réglages Système, « Confidentialité et sécurité », et cliquez sur « Ouvrir quand même ». |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | Ouvrez `about:debugging`, « Ce Firefox », « Charger un module complémentaire temporaire », et choisissez le fichier. Après un redémarrage de Firefox, rechargez-le. |
| Chrome, Yandex Browser, Edge, Opera | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | Décompressez-le, ouvrez `chrome://extensions`, activez le « Mode développeur », appuyez sur « Charger l’extension non empaquetée » et choisissez le dossier ; gardez-le. Yandex Browser ne fait confiance qu’aux catalogues et peut désactiver l’extension. |

GitHub compile ces fichiers à partir du code de ce dépôt, installe chacun d’eux, lance le programme, puis seulement les publie. Ils n’ont pas de signature payante, Windows et macOS avertissent donc au premier lancement.

Vous pouvez aussi compiler le programme vous-même. Il faut Node.js 24 ou plus récent et Rust, sous Windows aussi Microsoft C++ Build Tools, sous Ubuntu aussi `libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev`.

```sh
git clone https://github.com/Ilia-Pero/just-screenshotter
cd just-screenshotter
npm ci
npm run tauri build
```

L’installateur apparaît dans `src-tauri/target/release/bundle`.

## Traductions

Les traductions sont automatiques et n’ont pas été relues par des locuteurs natifs. Si vous voyez une erreur, corrigez-la dans `locales` ou dans `docs`.

## Participer et licence

Comment compiler, tester et modifier le programme : [CONTRIBUTING.md](../CONTRIBUTING.md). La licence est [MIT](../LICENSE).

<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">Just Screenshotter</h1>

<p align="center">Στιγμιότυπα και εγγραφή οθόνης για Windows, macOS και Linux</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/just-screenshotter/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/just-screenshotter/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/just-screenshotter?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · **Ελληνικά** · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

![Ο επεξεργαστής πάνω από δείγμα οθόνης: μαρκαδόρος, οβάλ, βέλος και κείμενο](editor.png)

Με το πλήκτρο Print Screen το πρόγραμμα φωτογραφίζει όλες τις οθόνες και ανοίγει από πάνω έναν επεξεργαστή. Εκεί επιλέγετε μια περιοχή, σχεδιάζετε πάνω της και αντιγράφετε, αποθηκεύετε ή στέλνετε την εικόνα. Η ίδια περιοχή μπορεί να εγγραφεί ως βίντεο. Ανάμεσα στα στιγμιότυπα μένει στη μνήμη μία διεργασία με εικονίδιο στην περιοχή ειδοποιήσεων.

## Εγκατάσταση

| Σύστημα | Αρχείο | Πώς εγκαθίσταται |
| --- | --- | --- |
| Windows 10, 11 | [just-screenshotter-windows.exe](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-windows.exe) | Εκτελέστε το αρχείο. Αν τα Windows δείξουν προειδοποίηση, επιλέξτε «Περισσότερες πληροφορίες» και «Εκτέλεση ούτως ή άλλως». |
| macOS 10.13+ | [just-screenshotter-macos.dmg](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-macos.dmg) | Σύρετε το πρόγραμμα στις «Εφαρμογές» και ανοίξτε το. Μετά την πρώτη άρνηση του macOS ανοίξτε τις Ρυθμίσεις συστήματος, «Απόρρητο και ασφάλεια», και πατήστε «Άνοιγμα ούτως ή άλλως». |
| Ubuntu 24.04+ | [just-screenshotter-ubuntu.deb](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-ubuntu.deb) | `sudo apt install ./just-screenshotter-ubuntu.deb` |
| Firefox 128+ | [just-screenshotter-firefox.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-firefox.zip) | Ανοίξτε το `about:debugging`, «Αυτό το Firefox», «Φόρτωση προσωρινού προσθέτου» και επιλέξτε το αρχείο. Μετά από επανεκκίνηση του Firefox φορτώστε το ξανά. |
| Chrome, Edge, Opera, Yandex Browser | [just-screenshotter-chrome.zip](https://github.com/Ilia-Pero/just-screenshotter/releases/latest/download/just-screenshotter-chrome.zip) | Αποσυμπιέστε το αρχείο σε φάκελο που θα κρατήσετε. Ανοίξτε το `chrome://extensions`, ενεργοποιήστε τη λειτουργία προγραμματιστή, πατήστε «Φόρτωση μη συσκευασμένης» και επιλέξτε τον φάκελο. |

Τα Windows και το macOS προειδοποιούν επειδή τα αρχεία δεν έχουν πληρωμένη υπογραφή. Το GitHub τα φτιάχνει από αυτό το αποθετήριο, εγκαθιστά τα προγράμματα σε καθαρές μηχανές και τα εκκινεί πριν δημοσιευτεί η έκδοση.

## Δυνατότητες

- Σχεδίαση με μολύβι, γραμμή, βέλος, πλαίσιο, οβάλ, μαρκαδόρο και κείμενο: οκτώ χρώματα, πάχος από 1 έως 72 pixel.
- Ό,τι σχεδιάσατε μετακινείται, αλλάζει χρώμα ή μέγεθος, ξαναγράφεται ή διαγράφεται οποιαδήποτε στιγμή.
- Η εικόνα αντιγράφεται, αποθηκεύεται ως PNG ή στέλνεται στον [δικό σας διακομιστή](../server/README.md).
- Βίντεο μιας περιοχής με τον ήχο του υπολογιστή και το μικρόφωνο: MP4 σε Windows, MOV σε macOS, WebM σε Linux. Πριν από το macOS 13 εγγράφεται μόνο το μικρόφωνο.
- 14 γλώσσες· τα πλήκτρα και ο φάκελος ορίζονται στις ρυθμίσεις.

## Πλήκτρα

| Ενέργεια | Windows, Linux | macOS |
| --- | --- | --- |
| Στιγμιότυπο περιοχής | Print Screen | ⌘⇧9 |
| Στιγμιότυπο ολόκληρης της οθόνης | Shift + Print Screen | |
| Στιγμιότυπο του ενεργού παραθύρου, μόνο Windows | Alt + Print Screen | |
| Βίντεο περιοχής | Ctrl + Shift + Print Screen | ⌘⇧8 |

Τα πλήκτρα αλλάζουν στις ρυθμίσεις. Στον επεξεργαστή το Ctrl+C αντιγράφει, το Ctrl+S αποθηκεύει, το Ctrl+Z αναιρεί, το Esc κλείνει.

## Επέκταση προγράμματος περιήγησης

Φωτογραφίζει το ορατό τμήμα μιας σελίδας (Alt+Shift+S) ή ολόκληρη τη σελίδα (Alt+Shift+P) και ανοίγει τον ίδιο επεξεργαστή σε νέα καρτέλα. Λειτουργεί σε Firefox και σε προγράμματα περιήγησης με Chromium.

## Δεδομένα και δίκτυο

Οι εικόνες και οι εγγραφές μένουν στον υπολογιστή σας. Το πρόγραμμα χρησιμοποιεί το δίκτυο όταν στέλνετε μια εικόνα στον διακομιστή σας και όταν πατάτε «Έλεγχος για ενημερώσεις». Δεν υπάρχει λογαριασμός ούτε τηλεμετρία.

## Περισσότερα

Οι μετρήσεις, τα ελεγμένα συστήματα και η μεταγλώττιση περιγράφονται [στα αγγλικά](../README.md).

Το κείμενο μεταφράστηκε από μηχανή και δεν το έχουν ελέγξει φυσικοί ομιλητές. Οι διορθώσεις γίνονται στον φάκελο `docs`.

[MIT](../LICENSE)

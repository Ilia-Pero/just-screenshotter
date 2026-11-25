import type { Box } from './box.ts';
import type { Choice } from './brushes.ts';
import { PALETTE, RED } from './colors.ts';
import type { Ears, Tongue } from './desk.ts';
import { button, connected, dock, element, size, titled } from './elements.ts';
import { ICONS } from './icons.ts';
import { Layout } from './layout.ts';
import type { Session } from './session.ts';
import { THICKEST, THINNEST } from './thickness.ts';
import type { Tool } from './tool.ts';
import { Hand } from './tool.ts';
import type { Words } from './words.ts';
import { filled } from './words.ts';

/** What a person can do with the buttons. */
export interface Acts {
  copy(): void;
  save(): void;
  share(): void;
  leave(): void;
  undo(): void;
  drop(): void;
  equip(tool: Tool): void;
  dye(color: string): void;
  thick(width: number): void;
  listens(): Promise<Ears>;
  film(ears: Ears): void;
  languages(): Promise<ReadonlyArray<Tongue>>;
  speak(code: string): void;
}

/** A button, the key of its title, and the keys that do the same. */
interface Sign {
  readonly node: HTMLElement;
  readonly title: keyof Words;
  readonly tail: string;
}

/** A key combination the way this system writes it. */
function chord(letter: string): string {
  return /Mac/.test(navigator.platform) ? ` (⌘${letter})` : ` (Ctrl+${letter})`;
}

/**
 * Everything around the selection: buttons, the palette, the size and the messages.
 * unit is the number of screenshot pixels in one pixel of the window,
 * shares tells whether there is a server to upload to.
 */
export class Panels {
  private readonly root: HTMLElement;
  private readonly unit: number;
  private readonly shares: boolean;
  private readonly choices: ReadonlyArray<Choice>;
  private readonly badge = element('div', 'badge');
  private readonly hint = element('div', 'hint');
  private readonly alarm = element('div', 'alarm hidden');
  private readonly notice = element('div', 'notice hidden');
  private readonly tools = element('div', 'bar tools');
  private readonly actions = element('div', 'bar actions');
  private readonly colors = element('div', 'sheet');
  private readonly widths = element('div', 'sheet widths');
  private readonly tongues = element('div', 'sheet tongues');
  private readonly filming = element('div', 'sheet filming');
  private readonly sound = element('input', '');
  private readonly voice = element('input', '');
  private readonly dot = element('span', 'dot');
  private readonly slider = element('input', 'slider');
  private readonly width = element('span', 'width');
  private readonly undo = button(ICONS.undo);
  private readonly drop = button(ICONS.drop);
  private readonly gauge = button(ICONS.thickness);
  private readonly signs: Sign[] = [];
  private readonly texts: Sign[] = [];
  private readonly kit: Tool[];
  private words: Words;

  constructor(
    root: HTMLElement,
    words: Words,
    unit: number,
    shares: boolean,
    choices: ReadonlyArray<Choice>,
  ) {
    this.root = root;
    this.words = words;
    this.unit = unit;
    this.shares = shares;
    this.choices = choices;
    this.kit = choices.map((choice) => choice.tool);
  }

  /** Puts the panels into the window and connects the buttons. */
  build(acts: Acts): void {
    // Messages follow the direction of their language, while the panels stay where they are.
    [this.hint, this.alarm, this.notice].forEach((node) => {
      node.dir = 'auto';
    });
    this.brushKeys(acts).forEach((key) => this.tools.appendChild(key));
    this.tools.appendChild(this.colorWell(acts));
    this.tools.appendChild(this.widthWell(acts));
    connected(this.undo, () => acts.undo());
    connected(this.drop, () => acts.drop());
    this.tools.appendChild(this.signed(this.undo, 'undo', chord('Z')));
    this.tools.appendChild(this.signed(this.drop, 'drop', ' (Delete)'));
    this.actionKeys(acts).forEach((key) => this.actions.appendChild(key));
    [this.badge, this.hint, this.alarm, this.notice, this.tools, this.actions].forEach((node) =>
      this.root.appendChild(node),
    );
    this.speak(this.words);
  }

  /** Writes everything in the language of the words. */
  speak(words: Words): void {
    this.words = words;
    this.hint.textContent = words.hint;
    this.signs.forEach((sign) => titled(sign.node, words[sign.title] + sign.tail));
    this.texts.forEach((text) => {
      text.node.textContent = words[text.title];
    });
  }

  /** Shows the state of the session and moves the panels next to the selection. */
  show(session: Session, dragging: boolean): void {
    const box = session.place();
    this.undo.disabled = !session.begun();
    this.drop.disabled = !session.picked();
    this.dot.style.background = session.hue();
    this.measure(session);
    this.badge.textContent = `${box.width()} × ${box.height()}`;
    this.hint.classList.toggle('hidden', !box.blank());
    [this.badge, this.tools, this.actions].forEach((node) =>
      node.classList.toggle('hidden', box.blank()),
    );
    this.root.classList.toggle('dragging', dragging);
    this.arrange(box.scaled(1 / this.unit));
  }

  /** Removes everything temporary: what the buttons have opened, and the last error. */
  hush(): void {
    this.alarm.classList.add('hidden');
    [this.colors, this.widths, this.tongues, this.filming].forEach((sheet) =>
      sheet.classList.remove('open'),
    );
  }

  complain(error: unknown): void {
    this.notice.classList.add('hidden');
    this.alarm.textContent = filled(this.words.failure, 'reason', String(error));
    this.alarm.classList.remove('hidden');
  }

  /** Tells that the picture is on its way, if that takes time. */
  announce(long: boolean): void {
    this.notice.textContent = this.words.sharing;
    this.notice.classList.toggle('hidden', !long);
  }

  /** The button with the title in the current language. */
  private signed<T extends HTMLElement>(node: T, title: keyof Words, tail: string): T {
    this.signs.push({ node, title, tail });
    return node;
  }

  private brushKeys(acts: Acts): ReadonlyArray<HTMLButtonElement> {
    const keys: ReadonlyArray<HTMLButtonElement> = this.choices.map((choice, index) => {
      const key = button(choice.icon);
      connected(key, () => {
        const on = !key.classList.contains('on');
        keys.forEach((other) => other.classList.remove('on'));
        key.classList.toggle('on', on);
        acts.equip((on && this.kit[index]) || new Hand());
      });
      return this.signed(key, choice.title, '');
    });
    return keys;
  }

  private colorWell(acts: Acts): HTMLElement {
    const pick = connected(button(''), () => this.toggle(this.colors));
    this.dot.style.background = RED;
    PALETTE.forEach((color) => {
      const swatch = connected(button(''), () => {
        this.colors.classList.remove('open');
        acts.dye(color.value);
      });
      swatch.className = 'swatch';
      swatch.style.background = color.value;
      this.colors.appendChild(this.signed(swatch, color.title, ''));
    });
    pick.appendChild(this.dot);
    return this.well(this.signed(pick, 'color', ''), this.colors);
  }

  private widthWell(acts: Acts): HTMLElement {
    connected(this.gauge, () => this.toggle(this.widths));
    this.slider.type = 'range';
    this.slider.min = String(THINNEST);
    this.slider.max = String(THICKEST);
    this.slider.addEventListener('input', () =>
      acts.thick(Number(this.slider.value) * this.unit),
    );
    this.slider.addEventListener('change', () => this.slider.blur());
    this.widths.appendChild(this.signed(this.slider, 'thickness', ''));
    this.widths.appendChild(this.width);
    return this.well(this.signed(this.gauge, 'thickness', ''), this.widths);
  }

  private filmWell(acts: Acts): HTMLElement {
    const open = connected(button(ICONS.film), () => {
      acts
        .listens()
        .then((ears) => {
          this.sound.checked = ears.sound;
          this.sound.disabled = !ears.hears;
          this.voice.checked = ears.voice;
          this.unfold(this.filming);
        })
        .catch((error: unknown) => this.complain(error));
    });
    const start = connected(button(''), () =>
      acts.film({
        hears: !this.sound.disabled,
        sound: this.sound.checked,
        voice: this.voice.checked,
      }),
    );
    start.className = 'start';
    this.filming.appendChild(this.tick(this.sound, 'film.sound'));
    this.filming.appendChild(this.tick(this.voice, 'film.voice'));
    this.filming.appendChild(this.said(start, 'film.start'));
    return this.well(this.signed(open, 'film', ''), this.filming);
  }

  /** A box to tick, with its text. */
  private tick(box: HTMLInputElement, title: keyof Words): HTMLElement {
    const row = element('label', 'tick');
    box.type = 'checkbox';
    row.dir = 'auto';
    row.appendChild(box);
    row.appendChild(this.said(element('span', ''), title));
    return row;
  }

  /** The element with the text in the current language. */
  private said<T extends HTMLElement>(node: T, title: keyof Words): T {
    node.dir = 'auto';
    this.texts.push({ node, title, tail: '' });
    return node;
  }

  private tongueWell(acts: Acts): HTMLElement {
    const open = connected(button(ICONS.language), () => {
      acts
        .languages()
        .then((tongues) => this.offer(tongues, acts))
        .catch((error: unknown) => this.complain(error));
    });
    return this.well(this.signed(open, 'menu.language', ''), this.tongues);
  }

  /** Shows the languages, or hides them if they are shown. */
  private offer(tongues: ReadonlyArray<Tongue>, acts: Acts): void {
    this.tongues.textContent = '';
    tongues.forEach((tongue) => {
      const row = connected(button(''), () => {
        this.tongues.classList.remove('open');
        acts.speak(tongue.code);
      });
      row.className = tongue.on ? 'tongue on' : 'tongue';
      row.dir = 'auto';
      row.textContent = tongue.title;
      this.tongues.appendChild(row);
    });
    this.unfold(this.tongues);
  }

  /** Shows the sheet above the buttons, or below them if there is no room; hides a shown one. */
  private unfold(sheet: HTMLElement): void {
    this.toggle(sheet);
    const floor = this.actions.getBoundingClientRect().top;
    sheet.classList.toggle('below', floor < sheet.offsetHeight + 16);
  }

  private well(key: HTMLElement, sheet: HTMLElement): HTMLElement {
    const well = element('div', 'well');
    well.appendChild(key);
    well.appendChild(sheet);
    return well;
  }

  private toggle(sheet: HTMLElement): void {
    const open = !sheet.classList.contains('open');
    this.hush();
    sheet.classList.toggle('open', open);
  }

  private actionKeys(acts: Acts): ReadonlyArray<HTMLElement> {
    const keys: HTMLElement[] = [
      this.signed(
        connected(button(ICONS.copy), () => acts.copy()),
        'copy',
        chord('C'),
      ),
      this.signed(
        connected(button(ICONS.save), () => acts.save()),
        'save',
        chord('S'),
      ),
    ];
    if (this.shares) {
      keys.push(
        this.signed(
          connected(button(ICONS.link), () => acts.share()),
          'share',
          chord('U'),
        ),
      );
    }
    keys.push(
      this.filmWell(acts),
      this.tongueWell(acts),
      this.signed(
        connected(button(ICONS.close), () => acts.leave()),
        'close',
        ' (Esc)',
      ),
    );
    return keys;
  }

  /** Shows the thickness. The tool in use remembers it, so that it returns with the tool. */
  private measure(session: Session): void {
    const width = session.thickness() / this.unit;
    this.gauge.disabled = width === 0;
    if (width === 0) {
      this.widths.classList.remove('open');
    } else {
      this.slider.value = String(width);
      this.width.textContent = String(Math.round(width));
    }
    this.kit.forEach((tool, index) => {
      const key = this.tools.children[index];
      if (key !== undefined && key.classList.contains('on')) {
        this.kit[index] = tool.thick(session.strength());
      }
    });
  }

  private arrange(area: Box): void {
    const layout = new Layout(
      { width: this.root.clientWidth, height: this.root.clientHeight },
      area,
      size(this.tools),
      size(this.actions),
      size(this.badge),
    );
    dock(this.tools, layout.toolsSpot());
    dock(this.actions, layout.actionsSpot());
    dock(this.badge, layout.badgeSpot());
  }
}

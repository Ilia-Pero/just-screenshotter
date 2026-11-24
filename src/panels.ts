import type { Box } from './box.ts';
import { brushes } from './brushes.ts';
import { RED, palette } from './colors.ts';
import { ICONS } from './icons.ts';
import { Layout } from './layout.ts';
import type { Point } from './point.ts';
import type { Session } from './session.ts';
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
  equip(tool: Tool): void;
  dye(color: string): void;
}

function element<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  name: string,
): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  node.className = name;
  return node;
}

function button(title: string, icon: string, act: () => void): HTMLButtonElement {
  const node = element('button', 'key');
  node.type = 'button';
  node.title = title;
  node.setAttribute('aria-label', title);
  node.innerHTML = icon;
  node.addEventListener('click', () => {
    act();
    node.blur();
  });
  return node;
}

function dock(node: HTMLElement, spot: Point): void {
  node.style.left = `${spot.x}px`;
  node.style.top = `${spot.y}px`;
}

function size(node: HTMLElement): { readonly width: number; readonly height: number } {
  return { width: node.offsetWidth, height: node.offsetHeight };
}

/** A key combination the way this system writes it. */
function chord(letter: string): string {
  return /Mac/.test(navigator.platform) ? `⌘${letter}` : `Ctrl+${letter}`;
}

/**
 * Everything around the selection: buttons, the palette, the size and the messages.
 * unit is the number of screenshot pixels in one pixel of the window,
 * shares tells whether there is a server to upload to.
 */
export class Panels {
  private readonly root: HTMLElement;
  private readonly words: Words;
  private readonly unit: number;
  private readonly shares: boolean;
  private readonly badge = element('div', 'badge');
  private readonly hint = element('div', 'hint');
  private readonly alarm = element('div', 'alarm hidden');
  private readonly notice = element('div', 'notice hidden');
  private readonly tools = element('div', 'bar tools');
  private readonly actions = element('div', 'bar actions');
  private readonly colors = element('div', 'palette');
  private readonly dot = element('span', 'dot');
  private readonly undo = element('button', 'key');

  constructor(root: HTMLElement, words: Words, unit: number, shares: boolean) {
    this.root = root;
    this.words = words;
    this.unit = unit;
    this.shares = shares;
  }

  /** Puts the panels into the window and connects the buttons. */
  build(acts: Acts): void {
    this.hint.textContent = this.words.hint;
    // Messages follow the direction of their language, while the panels stay where they are.
    [this.hint, this.alarm, this.notice].forEach((node) => {
      node.dir = 'auto';
    });
    this.brushKeys(acts).forEach((key) => this.tools.appendChild(key));
    this.tools.appendChild(this.well(acts));
    this.tools.appendChild(this.undoKey(acts));
    this.actionKeys(acts).forEach((key) => this.actions.appendChild(key));
    [this.badge, this.hint, this.alarm, this.notice, this.tools, this.actions].forEach((node) =>
      this.root.appendChild(node),
    );
  }

  /** Shows the state of the session and moves the panels next to the selection. */
  show(session: Session, dragging: boolean): void {
    const box = session.place();
    this.undo.disabled = !session.marked();
    this.badge.textContent = `${box.width()} × ${box.height()}`;
    this.hint.classList.toggle('hidden', !box.blank());
    [this.badge, this.tools, this.actions].forEach((node) =>
      node.classList.toggle('hidden', box.blank()),
    );
    this.root.classList.toggle('dragging', dragging);
    this.arrange(box.scaled(1 / this.unit));
  }

  /** Removes everything temporary: the palette and the last error. */
  hush(): void {
    this.alarm.classList.add('hidden');
    this.colors.classList.remove('open');
  }

  complain(error: unknown): void {
    this.notice.classList.add('hidden');
    this.alarm.textContent = filled(this.words.failure, 'reason', String(error));
    this.alarm.classList.remove('hidden');
  }

  announce(text: string): void {
    this.notice.textContent = text;
    this.notice.classList.toggle('hidden', text === '');
  }

  private brushKeys(acts: Acts): ReadonlyArray<HTMLButtonElement> {
    const hand = new Hand();
    const keys: ReadonlyArray<HTMLButtonElement> = brushes(this.words, this.unit).map((brush) => {
      const key = button(brush.title, brush.icon, () => {
        const on = !key.classList.contains('on');
        keys.forEach((other) => other.classList.remove('on'));
        key.classList.toggle('on', on);
        acts.equip(on ? brush.tool : hand);
      });
      return key;
    });
    return keys;
  }

  private well(acts: Acts): HTMLElement {
    const well = element('div', 'well');
    const pick = button(this.words.color, '', () => this.colors.classList.toggle('open'));
    this.dot.style.background = RED;
    palette(this.words).forEach((color) => {
      const swatch = button(color.title, '', () => {
        this.dot.style.background = color.value;
        this.colors.classList.remove('open');
        acts.dye(color.value);
      });
      swatch.className = 'swatch';
      swatch.style.background = color.value;
      this.colors.appendChild(swatch);
    });
    pick.appendChild(this.dot);
    well.appendChild(pick);
    well.appendChild(this.colors);
    return well;
  }

  private undoKey(acts: Acts): HTMLButtonElement {
    const title = `${this.words.undo} (${chord('Z')})`;
    this.undo.type = 'button';
    this.undo.title = title;
    this.undo.setAttribute('aria-label', title);
    this.undo.innerHTML = ICONS.undo;
    this.undo.addEventListener('click', () => {
      acts.undo();
      this.undo.blur();
    });
    return this.undo;
  }

  private actionKeys(acts: Acts): ReadonlyArray<HTMLButtonElement> {
    const keys = [
      button(`${this.words.copy} (${chord('C')})`, ICONS.copy, () => acts.copy()),
      button(`${this.words.save} (${chord('S')})`, ICONS.save, () => acts.save()),
    ];
    if (this.shares) {
      keys.push(button(`${this.words.share} (${chord('U')})`, ICONS.link, () => acts.share()));
    }
    keys.push(button(`${this.words.close} (Esc)`, ICONS.close, () => acts.leave()));
    return keys;
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

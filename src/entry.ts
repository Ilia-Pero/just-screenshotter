import type { Ink } from './ink.ts';
import {
  ENTER,
  ESCAPE,
  LETTER_C,
  LETTER_S,
  LETTER_U,
  LETTER_Z,
  PAD_ENTER,
} from './key.ts';
import type { Point } from './point.ts';
import type { Typing, Written } from './shape.ts';
import { Wheel, turned } from './thickness.ts';

/** Keys of the editor that work while a text is typed. typed tells whether a text needs them too. */
const PASSED = [
  { key: LETTER_S, typed: false },
  { key: LETTER_U, typed: false },
  { key: LETTER_C, typed: true },
  { key: LETTER_Z, typed: true },
];

/**
 * The field over the screenshot where a label is typed. It is as large as its text.
 * unit is the number of screenshot pixels in one pixel of the window.
 */
export class Entry implements Typing {
  private readonly layer: HTMLElement;
  private readonly unit: number;

  constructor(layer: HTMLElement, unit: number) {
    this.layer = layer;
    this.unit = unit;
  }

  /**
   * The typed text. Enter begins a new line; a click elsewhere, Esc, or Enter with Ctrl ends the
   * text. The wheel changes the size of the letters.
   */
  words(at: Point, ink: Ink, text: string): Promise<Written> {
    return new Promise((resolve) => {
      const field = document.createElement('textarea');
      const wheel = new Wheel();
      let size = ink.times(1 / this.unit);
      // Removing the field fires blur, so the second call has to be ignored.
      let open = true;
      const fit = (): void => this.fit(field, size);
      const finish = (): void => {
        if (open) {
          open = false;
          document.removeEventListener('mousedown', outside, true);
          document.removeEventListener('wheel', roll, true);
          resolve({
            text: field.value.replace(/\s+$/, ''),
            ink: ink.thick(size * this.unit),
          });
          field.remove();
        }
      };
      // Besides blur: a window without the focus does not fire it.
      const outside = (event: MouseEvent): void => {
        if (event.target !== field) {
          finish();
        }
      };
      const roll = (event: WheelEvent): void => {
        event.preventDefault();
        event.stopPropagation();
        size = turned(size, -wheel.notches(event));
        fit();
      };
      field.className = 'entry';
      field.rows = 1;
      field.wrap = 'off';
      field.spellcheck = true;
      field.value = text;
      field.style.left = `${at.x / this.unit}px`;
      field.style.top = `${at.y / this.unit}px`;
      field.style.color = ink.css();
      field.addEventListener('input', fit);
      field.addEventListener('keydown', (event) => {
        const held = event.ctrlKey || event.metaKey;
        const entered = ENTER.struck(event) || PAD_ENTER.struck(event);
        const blank = field.value === '';
        if (held && PASSED.some((one) => one.key.struck(event) && (blank || !one.typed))) {
          // The text is ended, and the key goes on to the editor.
          finish();
          return;
        }
        event.stopPropagation();
        if (ESCAPE.struck(event) || (held && entered)) {
          event.preventDefault();
          finish();
        }
      });
      // The menu of the system corrects spelling. The window itself has no menu.
      field.addEventListener('contextmenu', (event) => event.stopPropagation());
      // The field keeps the focus while a menu is open or another window is in front.
      field.addEventListener('blur', () => {
        if (document.activeElement !== field) {
          finish();
        }
      });
      document.addEventListener('mousedown', outside, true);
      document.addEventListener('wheel', roll, { capture: true, passive: false });
      this.layer.appendChild(field);
      fit();
      field.focus();
      field.select();
    });
  }

  /** Makes the field as large as its text, but not larger than the rest of the window. */
  private fit(field: HTMLTextAreaElement, size: number): void {
    field.style.fontSize = `${size}px`;
    field.style.width = '0';
    field.style.height = '0';
    field.style.width = `${Math.min(
      field.scrollWidth + size,
      this.layer.clientWidth - field.offsetLeft,
    )}px`;
    field.style.height = `${field.scrollHeight}px`;
  }
}

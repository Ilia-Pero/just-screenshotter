import type { Ink } from './ink.ts';
import { ENTER, ESCAPE, PAD_ENTER } from './key.ts';
import type { Point } from './point.ts';
import type { Typing } from './shape.ts';

/**
 * The input field over the screenshot where a label is typed.
 * unit is the number of screenshot pixels in one pixel of the window.
 */
export class Entry implements Typing {
  private readonly layer: HTMLElement;
  private readonly unit: number;

  constructor(layer: HTMLElement, unit: number) {
    this.layer = layer;
    this.unit = unit;
  }

  /** The typed text. Enter or a click elsewhere accepts it, Esc returns the text as it was. */
  words(at: Point, ink: Ink, text: string): Promise<string> {
    return new Promise((resolve) => {
      const field = document.createElement('input');
      // Removing the field fires blur, so the second call has to be ignored.
      let open = true;
      const finish = (typed: string): void => {
        if (open) {
          open = false;
          resolve(typed);
          field.remove();
        }
      };
      field.type = 'text';
      field.className = 'entry';
      field.value = text;
      field.style.left = `${at.x / this.unit}px`;
      field.style.top = `${at.y / this.unit}px`;
      field.style.fontSize = `${ink.times(1 / this.unit)}px`;
      field.style.color = ink.css();
      field.addEventListener('keydown', (event) => {
        event.stopPropagation();
        if (ENTER.struck(event) || PAD_ENTER.struck(event)) {
          finish(field.value);
        }
        if (ESCAPE.struck(event)) {
          finish(text);
        }
      });
      field.addEventListener('blur', () => finish(field.value));
      this.layer.appendChild(field);
      field.focus();
      field.select();
    });
  }
}

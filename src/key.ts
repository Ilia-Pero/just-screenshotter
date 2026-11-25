/** A key press as the window reports it. KeyboardEvent looks like this. */
export interface Press {
  readonly code: string;
  readonly keyCode: number;
}

/**
 * A key of the keyboard: its name by position, and its legacy numeric code.
 * The name does not depend on the layout, but it is empty for presses sent by programs,
 * remote desktops and on-screen keyboards. Then the key is recognised by the number.
 */
export class Key {
  private readonly code: string;
  private readonly legacy: number;

  constructor(code: string, legacy: number) {
    this.code = code;
    this.legacy = legacy;
  }

  /** True if this very key is pressed. */
  struck(press: Press): boolean {
    return press.code === this.code || (press.code === '' && press.keyCode === this.legacy);
  }
}

export const ESCAPE = new Key('Escape', 27);
export const ENTER = new Key('Enter', 13);
export const PAD_ENTER = new Key('NumpadEnter', 13);
export const DELETE = new Key('Delete', 46);
export const BACKSPACE = new Key('Backspace', 8);
export const LETTER_C = new Key('KeyC', 67);
export const LETTER_S = new Key('KeyS', 83);
export const LETTER_U = new Key('KeyU', 85);
export const LETTER_Z = new Key('KeyZ', 90);

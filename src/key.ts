/** A key press as the window reports it. KeyboardEvent looks like this. */
export interface Press {
  readonly code: string;
  readonly key: string;
  readonly keyCode: number;
}

/** Something a press can be. */
export interface Strike {
  /** True if this very key is pressed. */
  struck(press: Press): boolean;
}

/**
 * A key of the keyboard: its name by position, and its legacy numeric code.
 * The name does not depend on the layout, but it is empty for presses sent by programs,
 * remote desktops and on-screen keyboards. Then the key is recognised by the number.
 */
export class Key implements Strike {
  private readonly code: string;
  private readonly legacy: number;

  constructor(code: string, legacy: number) {
    this.code = code;
    this.legacy = legacy;
  }

  struck(press: Press): boolean {
    return press.code === this.code || (press.code === '' && press.keyCode === this.legacy);
  }
}

/**
 * A key with a Latin letter. Layouts put letters in different places: on a German keyboard Z is
 * where others have Y. A layout of another alphabet has no such letter at all, and then the key is
 * the one that has it in the American layout.
 */
export class Letter implements Strike {
  private readonly letter: string;

  constructor(letter: string) {
    this.letter = letter;
  }

  struck(press: Press): boolean {
    return /^[a-z]$/i.test(press.key)
      ? press.key.toUpperCase() === this.letter
      : new Key(`Key${this.letter}`, this.letter.charCodeAt(0)).struck(press);
  }
}

export const ESCAPE = new Key('Escape', 27);
export const ENTER = new Key('Enter', 13);
export const PAD_ENTER = new Key('NumpadEnter', 13);
export const DELETE = new Key('Delete', 46);
export const BACKSPACE = new Key('Backspace', 8);
export const LETTER_C = new Letter('C');
export const LETTER_S = new Letter('S');
export const LETTER_U = new Letter('U');
export const LETTER_Z = new Letter('Z');

/** Keys a text is edited with while Ctrl is held. */
const EDITING: ReadonlyArray<Strike> = [
  new Letter('A'),
  LETTER_C,
  new Letter('V'),
  new Letter('X'),
  new Letter('Y'),
  LETTER_Z,
  ENTER,
  PAD_ENTER,
  DELETE,
  BACKSPACE,
  new Key('Insert', 45),
  new Key('Home', 36),
  new Key('End', 35),
  new Key('ArrowLeft', 37),
  new Key('ArrowUp', 38),
  new Key('ArrowRight', 39),
  new Key('ArrowDown', 40),
];

/** A press of a key together with the ones that are held. */
export interface Combination extends Press {
  readonly ctrlKey: boolean;
  readonly altKey: boolean;
  readonly metaKey: boolean;
}

/**
 * Whether a browser would do something of its own: F5 and Ctrl+R load the page anew, Ctrl+P prints
 * it. With Alt it would not: Alt+F4 closes the window, and Ctrl+Alt types signs in many layouts.
 */
export function browsing(press: Combination): boolean {
  if (press.altKey) {
    return false;
  }
  return (
    /^F\d+$/.test(press.code) ||
    ((press.ctrlKey || press.metaKey) && !EDITING.some((key) => key.struck(press)))
  );
}

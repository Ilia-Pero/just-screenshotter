/** A press of a key together with the keys that are held. KeyboardEvent looks like this. */
export interface Struck {
  readonly code: string;
  readonly key: string;
  readonly ctrlKey: boolean;
  readonly shiftKey: boolean;
  readonly altKey: boolean;
  readonly metaKey: boolean;
}

/** Keys that mean something only when they are held with others. */
const HELD = /^(?:Control|Shift|Alt|Meta|OS)(?:Left|Right)?$/u;

/** Keys that nobody types a text with: they may start an action alone. */
const ALONE = /^(?:PrintScreen|Pause|ScrollLock|Insert|F\d{1,2})$/u;

/** What the shell knows keys by. */
const KNOWN = /^(?:Key[A-Z]|Digit\d|Numpad\d|F\d{1,2}|PrintScreen|Pause|ScrollLock|Insert|Delete|Home|End|PageUp|PageDown|Arrow(?:Up|Down|Left|Right)|Space|Backquote|Minus|Equal|Comma|Period|Slash|Semicolon|Quote|Backslash|BracketLeft|BracketRight)$/u;

/**
 * Keys pressed together that a person wants to start an action with.
 * The name of the key does not depend on the layout, except when it is empty: programs, remote
 * desktops and on-screen keyboards send such presses. Windows and X11 know a letter by what is
 * written on the key, and macOS knows it by the place of the key.
 */
export class Chord {
  private readonly struck: Struck;
  private readonly placed: boolean;

  /** placed tells whether the system knows letters by their places. */
  constructor(struck: Struck, placed: boolean) {
    this.struck = struck;
    this.placed = placed;
  }

  /** True while only keys that are held have been pressed. */
  pending(): boolean {
    return HELD.test(this.name());
  }

  /** True if an action can be started with these keys without getting in the way of typing. */
  fit(): boolean {
    const name = this.name();
    const held = this.struck.ctrlKey || this.struck.altKey || this.struck.metaKey;
    return KNOWN.test(name) && (ALONE.test(name) || held);
  }

  /** The keys as the shell spells them. */
  spelled(): string {
    return [
      this.struck.ctrlKey ? 'Ctrl' : '',
      this.struck.altKey ? 'Alt' : '',
      this.struck.shiftKey ? 'Shift' : '',
      this.struck.metaKey ? 'Cmd' : '',
      this.name().replace(/^(?:Key|Digit)/u, ''),
    ]
      .filter((part) => part !== '')
      .join('+');
  }

  private name(): string {
    if (!this.placed && /^[a-z]$/iu.test(this.struck.key)) {
      return `Key${this.struck.key.toUpperCase()}`;
    }
    return this.struck.code === '' ? this.struck.key : this.struck.code;
  }
}

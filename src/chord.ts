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
 * desktops and on-screen keyboards send such presses.
 */
export class Chord {
  private readonly struck: Struck;

  constructor(struck: Struck) {
    this.struck = struck;
  }

  /** True while only keys that are held have been pressed. */
  pending(): boolean {
    return HELD.test(this.name());
  }

  /** True if an action can be started with these keys without getting in the way of typing. */
  fit(): boolean {
    const name = this.name();
    const { ctrlKey, altKey, metaKey } = this.struck;
    return KNOWN.test(name) && (ALONE.test(name) || ctrlKey || altKey || metaKey);
  }

  /** The keys as the shell spells them. */
  spelled(): string {
    const { ctrlKey, altKey, shiftKey, metaKey } = this.struck;
    return [
      ctrlKey ? 'Ctrl' : '',
      altKey ? 'Alt' : '',
      shiftKey ? 'Shift' : '',
      metaKey ? 'Cmd' : '',
      this.name().replace(/^(?:Key|Digit)/u, ''),
    ]
      .filter((part) => part !== '')
      .join('+');
  }

  private name(): string {
    return this.struck.code === '' ? this.struck.key : this.struck.code;
  }
}

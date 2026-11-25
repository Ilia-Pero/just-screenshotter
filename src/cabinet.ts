import type { Tongue } from './desk.ts';
import type { Words } from './words.ts';

/** An action and what starts it. */
export interface Row {
  readonly code: string;
  /** The key as the shell spells it. */
  readonly key: string;
  /** The key for a person. */
  readonly title: string;
  readonly mouse: string;
  /** Whether another program uses the key. */
  readonly busy: boolean;
}

/** Everything the window with the settings shows. */
export interface State {
  readonly acts: ReadonlyArray<Row>;
  /** Whether the program can assign keys by itself. */
  readonly assignable: boolean;
  /** Whether buttons of the mouse can start actions. */
  readonly mice: boolean;
  readonly folder: string;
  readonly ears: { readonly hears: boolean; readonly sound: boolean; readonly voice: boolean };
  /** Whether the program starts with the system. */
  readonly launch: boolean;
  readonly version: string;
}

/** What a person has chosen for an action. */
export interface Chosen {
  readonly code: string;
  readonly key: string;
  readonly mouse: string;
}

/** The latest release of the program. */
export interface Release {
  readonly version: string;
  /** Whether it is newer than this program. */
  readonly newer: boolean;
}

/** Where the settings are kept: in the shell, or in a browser tab for a look. */
export interface Cabinet {
  words(): Promise<Words>;
  state(): Promise<State>;
  /** Remembers what starts actions. */
  arrange(chosen: ReadonlyArray<Chosen>): Promise<State>;
  /** Lets keys and buttons go while a person presses the ones they want. */
  hold(on: boolean): Promise<void>;
  launch(on: boolean): Promise<State>;
  hear(sound: boolean, voice: boolean): Promise<State>;
  relocate(): Promise<State>;
  folder(): Promise<void>;
  uploading(): Promise<void>;
  languages(): Promise<ReadonlyArray<Tongue>>;
  speak(code: string): Promise<Words>;
  /** Looks for a newer version. */
  update(): Promise<Release>;
  /** Downloads the installer of the latest release. */
  download(): Promise<void>;
  /** Shows what is new in the latest release. */
  news(): Promise<void>;
}

/** The choice of a person put among the others. */
export function chosen(
  rows: ReadonlyArray<Row>,
  code: string,
  change: { readonly key?: string; readonly mouse?: string },
): ReadonlyArray<Chosen> {
  return rows.map((row) =>
    Object.assign(
      { code: row.code, key: row.key, mouse: row.mouse },
      row.code === code ? change : {},
    ),
  );
}

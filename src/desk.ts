import type { Shot } from './shot.ts';
import type { Words } from './words.ts';

/** A language of the program. */
export interface Tongue {
  /** An empty code means the language of the system. */
  readonly code: string;
  readonly title: string;
  /** True if the program speaks it because a person has chosen so. */
  readonly on: boolean;
}

/**
 * The desktop the tool runs on. The editor does not know where it is:
 * in a Tauri window (Shell) or in a browser tab (Sandbox).
 */
export interface Desk {
  /** The screen at the moment the tool was called. */
  shot(): Promise<Shot>;
  /** Puts the picture on the clipboard. */
  copy(png: Blob): Promise<void>;
  /** Saves the picture to a file. */
  save(png: Blob): Promise<void>;
  /** Whether there is a server to upload the picture to. */
  shares(): Promise<boolean>;
  /** Uploads the picture and puts the link on the clipboard. */
  share(png: Blob): Promise<void>;
  /** Languages to choose from. */
  languages(): Promise<ReadonlyArray<Tongue>>;
  /** Makes the whole program speak the language with the code. Returns its words. */
  speak(code: string): Promise<Words>;
  /** Closes the editor. */
  leave(): Promise<void>;
}

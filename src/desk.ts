import type { Box } from './box.ts';
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

/** What is recorded together with video. */
export interface Ears {
  /** Whether the sound of the computer can be recorded at all. */
  readonly hears: boolean;
  readonly sound: boolean;
  readonly voice: boolean;
}

/** What the editor begins with. */
export interface Opening {
  /** The area that is selected already, in pixels of the screenshot. It may be blank. */
  readonly area: Box;
  /** Whether a film of the area is wanted. */
  readonly film: boolean;
}

/**
 * The desktop the tool runs on. The editor does not know where it is:
 * in a Tauri window (Shell) or in a browser tab (Sandbox).
 */
export interface Desk {
  /** The screen at the moment the tool was called. */
  shot(): Promise<Shot>;
  /** What to begin with. */
  opening(): Promise<Opening>;
  /** Puts the picture on the clipboard. */
  copy(png: Blob): Promise<void>;
  /** Saves the picture to a file. */
  save(png: Blob): Promise<void>;
  /** Whether there is a server to upload the picture to. */
  shares(): Promise<boolean>;
  /** Uploads the picture and puts the link on the clipboard. */
  share(png: Blob): Promise<void>;
  /** What can be recorded together with video, and what was recorded the last time. */
  listens(): Promise<Ears>;
  /** Closes the editor and films the area of the screen. The sheet is the whole screenshot. */
  film(area: Box, sheet: Box, ears: Ears): Promise<void>;
  /** Languages to choose from. */
  languages(): Promise<ReadonlyArray<Tongue>>;
  /** Makes the whole program speak the language with the code. Returns its words. */
  speak(code: string): Promise<Words>;
  /** Closes the editor. */
  leave(): Promise<void>;
}

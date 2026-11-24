import type { Shot } from './shot.ts';

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
  /** Closes the editor. */
  leave(): Promise<void>;
}

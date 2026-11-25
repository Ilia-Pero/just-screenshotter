import './bridge.ts';
import { ESCAPE } from './key.ts';
import { Shell } from './shell.ts';
import { start } from './start.ts';
import type { Words } from './words.ts';
import { filled } from './words.ts';

const bridge = window.__TAURI_INTERNALS__;

/** Explains why the editor has not opened. */
function apologise(words: Words, error: unknown): void {
  document.documentElement.lang = words.code;
  document.body.textContent = filled(words.broken, 'reason', String(error));
  window.addEventListener('keydown', (event) => {
    if (ESCAPE.struck(event)) {
      bridge.invoke<void>('leave').catch(() => window.close());
    }
  });
}

// The window is created hidden. The shell shows it on "reveal", when the editor is already drawn.
bridge
  .invoke<Words>('words')
  .then((words) =>
    start(new Shell(bridge), document.body, words).catch((error: unknown) =>
      apologise(words, error),
    ),
  )
  .then(() => bridge.invoke<void>('reveal'))
  .catch((error: unknown) => {
    document.body.textContent = String(error);
  });

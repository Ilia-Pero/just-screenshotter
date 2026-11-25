import './bridge.ts';
import { clock } from './clock.ts';
import type { Words } from './words.ts';

/** How filming is going. */
interface Progress {
  /** How long it has been going on, in milliseconds. */
  readonly time: number;
  /** Whether the sound of the computer was asked for but cannot be heard. */
  readonly deaf: boolean;
}

const bridge = window.__TAURI_INTERNALS__;

function part(name: string): HTMLElement {
  const node = document.createElement('span');
  node.className = name;
  document.body.appendChild(node);
  return node;
}

function open(words: Words, progress: Progress): void {
  const opened = Date.now() - progress.time;
  const spot = part('spot');
  const time = part('time');
  const stop = document.createElement('button');
  const show = (): void => {
    time.textContent = clock(Date.now() - opened);
  };
  spot.classList.toggle('deaf', progress.deaf);
  spot.title = progress.deaf ? words['film.deaf'] : words.filming;
  stop.type = 'button';
  stop.dir = 'auto';
  stop.textContent = words['film.stop'];
  stop.addEventListener('click', () => {
    stop.disabled = true;
    bridge.invoke<void>('cease').catch((error: unknown) => {
      time.textContent = String(error);
    });
  });
  document.body.appendChild(stop);
  document.body.addEventListener('mousedown', (event) => {
    if (event.target !== stop) {
      bridge.invoke<void>('drag').catch(() => {
        // The window stays where it is.
      });
    }
  });
  window.addEventListener('contextmenu', (event) => event.preventDefault());
  show();
  // Once a second, and only while filming: the window is destroyed with the film.
  window.setInterval(show, 1000);
}

Promise.all([bridge.invoke<Words>('words'), bridge.invoke<Progress>('progress')])
  .then((ready) => open(ready[0], ready[1]))
  .catch((error: unknown) => {
    document.body.textContent = String(error);
  });

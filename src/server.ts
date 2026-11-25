import './bridge.ts';
import type { Words } from './words.ts';

/** The server known now. The key itself never gets into the window. */
interface Known {
  readonly address: string;
  readonly token: boolean;
}

function part<T extends HTMLElement>(id: string, kind: new () => T): T {
  const node = document.getElementById(id);
  if (node instanceof kind) {
    return node;
  }
  throw new Error(`The window has no part named ${id}`);
}

const bridge = window.__TAURI_INTERNALS__;
const address = part('address', HTMLInputElement);
const token = part('token', HTMLInputElement);
const outcome = part('outcome', HTMLParagraphElement);

function say(text: string, bad: boolean): void {
  outcome.textContent = text;
  outcome.className = bad ? 'bad' : '';
}

function show(words: Words): Promise<void> {
  return bridge.invoke<Known>('server').then((known) => {
    address.value = known.address;
    token.value = '';
    token.placeholder = known.token ? words['server.kept'] : '';
  });
}

function open(words: Words): Promise<void> {
  document.documentElement.lang = words.code;
  document.documentElement.dir = words.direction;
  part('intro', HTMLParagraphElement).textContent = words['server.intro'];
  part('address-title', HTMLLabelElement).textContent = words['server.address'];
  part('token-title', HTMLLabelElement).textContent = words['server.token'];
  part('save', HTMLButtonElement).textContent = words['server.save'];
  part('form', HTMLFormElement).addEventListener('submit', (event) => {
    event.preventDefault();
    bridge
      .invoke<void>('appoint', { address: address.value, token: token.value })
      .then(() => show(words))
      .then(() => say(address.value === '' ? words['server.off'] : words['server.saved'], false))
      .catch((error: unknown) => say(String(error), true));
  });
  return show(words);
}

bridge
  .invoke<Words>('words')
  .then(open)
  .catch((error: unknown) => say(String(error), true));

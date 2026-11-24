import { spoken } from './languages.ts';
import { Sandbox } from './sandbox.ts';
import { start } from './start.ts';
import { filled } from './words.ts';

// The language may be given in the address: http://127.0.0.1:5391/?he
const words = spoken(window.location.search.slice(1) || navigator.language);

start(new Sandbox(document.body), document.body, words).catch((error: unknown) => {
  document.body.textContent = filled(words.broken, 'reason', String(error));
});

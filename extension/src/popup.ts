import { LANGUAGES, spoken } from '../../src/languages.ts';
import type { Wish } from './browser.ts';
import { browser } from './browser.ts';
import { choice, choose, words } from './tongue.ts';

function part<T extends HTMLElement>(id: string, kind: new () => T): T {
  const node = document.getElementById(id);
  if (node instanceof kind) {
    return node;
  }
  throw new Error(`The window has no part named ${id}`);
}

const buttons: ReadonlyArray<readonly [Wish, HTMLButtonElement]> = [
  ['visible', part('visible', HTMLButtonElement)],
  ['page', part('page', HTMLButtonElement)],
];
const language = part('language', HTMLSelectElement);
const alarm = part('alarm', HTMLParagraphElement);

function lock(busy: boolean): void {
  buttons.forEach(([, button]) => {
    button.disabled = busy;
  });
}

async function complain(): Promise<void> {
  alarm.textContent = (await words())['tab.refused'];
  alarm.hidden = false;
  lock(false);
}

async function show(): Promise<void> {
  const said = await words();
  document.documentElement.dir = said.direction;
  buttons.forEach(([wish, button]) => {
    button.textContent = said[`tab.${wish}`];
  });
  language.setAttribute('aria-label', said['menu.language']);
  language.textContent = '';
  [{ code: '', title: said['menu.system'] }]
    .concat(Object.keys(LANGUAGES).map((code) => ({ code, title: spoken(code).language })))
    .forEach((tongue) => {
      const option = document.createElement('option');
      option.value = tongue.code;
      option.textContent = tongue.title;
      language.appendChild(option);
    });
  language.value = await choice();
}

buttons.forEach(([wish, button]) =>
  button.addEventListener('click', () => {
    alarm.hidden = true;
    lock(true);
    browser.runtime
      .sendMessage({ take: wish })
      .then((reply) => (reply.reason === '' ? window.close() : complain()))
      .catch(complain);
  }),
);
language.addEventListener('change', () => {
  choose(language.value).then(show).catch(complain);
});
show().catch(complain);

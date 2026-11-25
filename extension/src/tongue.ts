import { LANGUAGES, spoken } from '../../src/languages.ts';
import type { Words } from '../../src/words.ts';
import { browser } from './browser.ts';

/** The code of the chosen language. Empty if there is no choice. */
export async function choice(): Promise<string> {
  const code = String((await browser.storage.local.get('language'))['language'] ?? '');
  return code in LANGUAGES ? code : '';
}

/** Remembers the choice. An empty code returns it to the browser. */
export function choose(code: string): Promise<void> {
  return browser.storage.local.set({ language: code });
}

/** Words of the chosen language; of the one of the browser if there is no choice. */
export async function words(): Promise<Words> {
  return spoken((await choice()) || navigator.language);
}

import type en from '../locales/en.json';

/**
 * Everything the program says to a person in one language. The shell gives the window the words
 * of the current language only, so that opening it does not take reading a dozen of them.
 */
export type Words = Readonly<typeof en>;

/** The text with the hole in it filled: the hole "reason" looks like {reason}. */
export function filled(text: string, hole: string, value: string): string {
  return text.split(`{${hole}}`).join(value);
}

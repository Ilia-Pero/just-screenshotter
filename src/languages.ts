import be from '../locales/be.json' with { type: 'json' };
import de from '../locales/de.json' with { type: 'json' };
import el from '../locales/el.json' with { type: 'json' };
import en from '../locales/en.json' with { type: 'json' };
import es from '../locales/es.json' with { type: 'json' };
import fr from '../locales/fr.json' with { type: 'json' };
import he from '../locales/he.json' with { type: 'json' };
import it from '../locales/it.json' with { type: 'json' };
import ja from '../locales/ja.json' with { type: 'json' };
import kk from '../locales/kk.json' with { type: 'json' };
import ko from '../locales/ko.json' with { type: 'json' };
import ru from '../locales/ru.json' with { type: 'json' };
import sr from '../locales/sr.json' with { type: 'json' };
import zh from '../locales/zh.json' with { type: 'json' };
import type { Words } from './words.ts';

/** All languages by their codes. For the demo page and tests. */
export const LANGUAGES: Readonly<Record<string, Words>> = {
  be,
  de,
  el,
  en,
  es,
  fr,
  he,
  it,
  ja,
  kk,
  ko,
  ru,
  sr,
  zh,
};

/** Words in the language with the tag, such as ru-RU. English if there is no such language. */
export function spoken(tag: string): Words {
  const words = LANGUAGES[tag.toLowerCase().replace(/[-_.].*$/, '')];
  return words === undefined ? en : words;
}

// Sends the extension to the catalog of Firefox. The key is in AMO_KEY and AMO_SECRET.
// node tools/catalog.ts 0.5.0 package.zip source.zip news.md
import { randomUUID } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { basename } from 'node:path';
import { argv, env, stdout } from 'node:process';
import { setTimeout } from 'node:timers/promises';
import { Catalog, Extension, Key } from './mozilla.ts';

const STORE = new URL('../extension/store/', import.meta.url);

/** A part of the key, which is kept out of the repository. */
function secret(name: string): string {
  const value = env[name];
  if (value === undefined || value === '') {
    throw new Error(`${name} is not set`);
  }
  return value;
}

function file(place: string | URL, name: string, type: string): File {
  return new File([readFileSync(place)], name, { type });
}

const [version = '', pack = '', source = '', news = ''] = argv.slice(2);
const key = new Key(secret('AMO_KEY'), secret('AMO_SECRET'));
const extension = new Extension(
  new Catalog(
    'https://addons.mozilla.org/api/v5/',
    () => key.pass(Math.floor(Date.now() / 1000), randomUUID()),
    fetch,
  ),
  JSON.parse(readFileSync(new URL('firefox.json', STORE), 'utf8')),
  () => setTimeout(5000),
);
const place = await extension.send({
  version,
  news: readFileSync(news, 'utf8').trim(),
  pack: file(pack, basename(pack), 'application/zip'),
  source: file(source, basename(source), 'application/zip'),
  logo: file(new URL('logo.png', STORE), 'logo.png', 'image/png'),
  screen: file(new URL('screen.png', STORE), 'screen.png', 'image/png'),
});
stdout.write(`${version}: ${place}\n`);

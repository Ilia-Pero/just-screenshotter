// Lays the extension out for browsers of two kinds: their manifests differ. The folders are made
// anew every time: extension/packed/firefox and extension/packed/chrome.
import { cpSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

interface Manifest {
  readonly background: { readonly scripts: ReadonlyArray<string>; readonly service_worker: string };
  readonly browser_specific_settings: unknown;
  readonly [key: string]: unknown;
}

const PLACE = fileURLToPath(new URL('../extension', import.meta.url));

/** The oldest Chromium that knows everything the extension uses. */
const OLDEST = '120';

function read(): Manifest {
  return JSON.parse(readFileSync(`${PLACE}/web/manifest.json`, 'utf8'));
}

/** Firefox runs the background as scripts. */
function firefox(manifest: Manifest): object {
  return { ...manifest, background: { scripts: manifest.background.scripts } };
}

/** Chrome, Yandex Browser, Edge and Opera run it as a worker, and settings of Firefox mean nothing. */
function chrome(manifest: Manifest): object {
  const { browser_specific_settings: _, ...rest } = manifest;
  return {
    ...rest,
    minimum_chrome_version: OLDEST,
    background: { service_worker: manifest.background.service_worker },
  };
}

rmSync(`${PLACE}/packed`, { recursive: true, force: true });
[
  { name: 'firefox', manifest: firefox(read()) },
  { name: 'chrome', manifest: chrome(read()) },
].forEach((kind) => {
  const folder = `${PLACE}/packed/${kind.name}`;
  cpSync(`${PLACE}/web`, folder, { recursive: true });
  writeFileSync(`${folder}/manifest.json`, `${JSON.stringify(kind.manifest, null, 2)}\n`);
});

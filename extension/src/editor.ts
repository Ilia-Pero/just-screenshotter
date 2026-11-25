import { Box } from '../../src/box.ts';
import type { Desk, Ears, Opening, Tongue } from '../../src/desk.ts';
import { LANGUAGES, spoken } from '../../src/languages.ts';
import { developed } from '../../src/photo.ts';
import { Point } from '../../src/point.ts';
import type { Shot } from '../../src/shot.ts';
import { start } from '../../src/start.ts';
import type { Words } from '../../src/words.ts';
import { filled } from '../../src/words.ts';
import type { Taken } from './browser.ts';
import { browser } from './browser.ts';
import { sheet } from './sheet.ts';
import { choice, choose, words } from './tongue.ts';

function two(number: number): string {
  return String(number).padStart(2, '0');
}

/** The moment as a part of a file name. */
function stamp(now: Date): string {
  const day = [now.getFullYear(), two(now.getMonth() + 1), two(now.getDate())].join('-');
  const time = [two(now.getHours()), two(now.getMinutes()), two(now.getSeconds())].join('.');
  return `${day} ${time}`;
}

/** A tab with the picture instead of the desktop. */
class Tab implements Desk {
  private readonly canvas: HTMLCanvasElement;
  private readonly taken: Taken;

  constructor(canvas: HTMLCanvasElement, taken: Taken) {
    this.canvas = canvas;
    this.taken = taken;
  }

  async shot(): Promise<Shot> {
    const png = await new Promise<Blob>((resolve, reject) => {
      this.canvas.toBlob((blob) =>
        blob === null ? reject(new Error('The picture is too large')) : resolve(blob),
      );
    });
    return developed(URL.createObjectURL(png));
  }

  /** The whole page is wanted as a whole, so it is selected. */
  opening(): Promise<Opening> {
    const size =
      this.taken.wish === 'page'
        ? new Point(this.canvas.width, this.canvas.height)
        : new Point(0, 0);
    return Promise.resolve({ area: new Box(new Point(0, 0), size), film: false });
  }

  copy(png: Blob): Promise<void> {
    return navigator.clipboard.write([new ClipboardItem({ 'image/png': png })]);
  }

  async save(png: Blob): Promise<void> {
    const link = document.createElement('a');
    link.href = URL.createObjectURL(png);
    link.download = `${(await words()).file} ${stamp(new Date())}.png`;
    link.click();
    // The browser reads the picture after the click, and the tab must not go before that.
    await new Promise((resolve) => {
      setTimeout(resolve, 1000);
    });
  }

  shares(): Promise<boolean> {
    return Promise.resolve(false);
  }

  share(): Promise<void> {
    return Promise.reject(new Error('A tab has no server'));
  }

  films(): Promise<boolean> {
    return Promise.resolve(false);
  }

  listens(): Promise<Ears> {
    return Promise.resolve({ hears: false, sound: false, voice: false });
  }

  film(): Promise<void> {
    return Promise.reject(new Error('A tab does not film'));
  }

  async languages(): Promise<ReadonlyArray<Tongue>> {
    const chosen = await choice();
    const system = { code: '', title: (await words())['menu.system'], on: chosen === '' };
    return [system].concat(
      Object.keys(LANGUAGES).map((code) => ({
        code,
        title: spoken(code).language,
        on: code === chosen,
      })),
    );
  }

  async speak(code: string): Promise<Words> {
    await choose(code);
    return words();
  }

  async leave(): Promise<void> {
    await browser.tabs.remove((await browser.tabs.getCurrent()).id);
  }
}

async function open(): Promise<void> {
  const said = await words();
  const reply = await browser.runtime.sendMessage({ give: window.location.hash.slice(1) });
  const [taken] = reply.taken;
  if (taken === undefined) {
    throw new Error(said['tab.gone']);
  }
  const canvas = await sheet(taken);
  document.title = said.name;
  document.body.style.width = `${canvas.width / taken.ratio}px`;
  document.body.style.height = `${canvas.height / taken.ratio}px`;
  await start(new Tab(canvas, taken), document.body, said);
  if (taken.cut) {
    const note = document.createElement('div');
    note.className = 'alarm';
    note.textContent = said['tab.long'];
    document.body.appendChild(note);
  }
}

open().catch((error: unknown) => {
  words()
    .then((said) => {
      document.body.className = 'broken';
      document.body.textContent = filled(said.broken, 'reason', String(error));
    })
    .catch(() => {
      document.body.textContent = String(error);
    });
});

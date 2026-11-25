import type { Piece, Tab, Taken, Wish } from './browser.ts';
import { browser, unhurried } from './browser.ts';
import { Strips } from './strips.ts';

/** Browsers draw on no taller pictures. */
const LONGEST = 32_000;

/** How long a page is given to draw itself after it is scrolled, in milliseconds. */
const PAUSE = unhurried ? 550 : 150;

/** The size of a page and of what is seen of it, without the bars it is scrolled with. */
interface Measure {
  readonly width: number;
  readonly height: number;
  readonly view: number;
  readonly ratio: number;
  readonly top: number;
}

/** The page as it is before it is scrolled. Functions for a page have to be whole in themselves. */
function measured(): Measure {
  const root = document.scrollingElement ?? document.documentElement;
  return {
    width: root.clientWidth,
    height: root.scrollHeight,
    view: root.clientHeight,
    ratio: window.devicePixelRatio,
    top: window.scrollY,
  };
}

/**
 * Scrolls the page and tells how far it has gone. What stays in its place on the screen would be
 * on every piece: it is left on the first one only, and sticky parts return to theirs.
 */
function scrolled(top: number, first: boolean): number {
  if (!first && document.querySelector('[data-just-screenshotter]') === null) {
    document.querySelectorAll<HTMLElement>('body *').forEach((node) => {
      const position = getComputedStyle(node).position;
      if (position === 'fixed' || position === 'sticky') {
        node.setAttribute('data-just-screenshotter', node.getAttribute('style') ?? '');
        if (position === 'fixed') {
          node.style.setProperty('visibility', 'hidden', 'important');
        } else {
          node.style.setProperty('position', 'static', 'important');
        }
      }
    });
  }
  window.scrollTo({ top, left: 0, behavior: 'instant' });
  return window.scrollY;
}

/** Returns the page to what it was. */
function returned(top: number): void {
  document.querySelectorAll<HTMLElement>('[data-just-screenshotter]').forEach((node) => {
    const style = node.getAttribute('data-just-screenshotter') ?? '';
    node.removeAttribute('data-just-screenshotter');
    if (style === '') {
      node.removeAttribute('style');
    } else {
      node.setAttribute('style', style);
    }
  });
  window.scrollTo({ top, left: 0, behavior: 'instant' });
}

async function run<A extends ReadonlyArray<unknown>, R>(
  tab: Tab,
  func: (...args: A) => R,
  ...args: A
): Promise<R> {
  const [outcome] = await browser.scripting.executeScript({
    target: { tabId: tab.id },
    func,
    args,
  });
  if (outcome === undefined) {
    throw new Error('The page has not answered');
  }
  return outcome.result;
}

function snap(tab: Tab): Promise<string> {
  return browser.tabs.captureVisibleTab(tab.windowId, { format: 'png' });
}

function pause(): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, PAUSE);
  });
}

/** Takes a picture of what is seen of the page. */
async function visible(tab: Tab, wish: Wish): Promise<Taken> {
  const { width, view, ratio } = await run(tab, measured);
  const url = await snap(tab);
  return { pieces: [{ url, top: 0 }], width, height: view, ratio, cut: false, wish };
}

/** Takes a picture of the whole page, strip by strip. */
async function page(tab: Tab, wish: Wish): Promise<Taken> {
  const { width, height, view, ratio, top } = await run(tab, measured);
  const limit = Math.floor(LONGEST / ratio);
  const taken = Math.min(height, limit);
  const pieces: Piece[] = [];
  try {
    for (const strip of new Strips(taken, view).tops()) {
      // One after another: the page is one, and it is scrolled.
      // oxlint-disable no-await-in-loop
      const reached = await run(tab, scrolled, strip, pieces.length === 0);
      await pause();
      pieces.push({ url: await snap(tab), top: reached });
      // oxlint-enable no-await-in-loop
    }
  } finally {
    await run(tab, returned, top);
  }
  return { pieces, width, height: taken, ratio, cut: height > limit, wish };
}

/** Takes a picture of the page a person looks at, and opens the editor next to it. */
export async function take(wish: Wish, keep: (taken: Taken) => string): Promise<void> {
  const [tab] = await browser.tabs.query({ active: true, lastFocusedWindow: true });
  if (tab === undefined) {
    throw new Error('There is no page');
  }
  const taken = await (wish === 'page' ? page(tab, wish) : visible(tab, wish));
  await browser.tabs.create({
    url: browser.runtime.getURL(`editor.html#${keep(taken)}`),
    index: tab.index + 1,
  });
}

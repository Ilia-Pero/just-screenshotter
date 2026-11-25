/** What the extension needs from the browser. */
export interface Browser {
  readonly action: {
    setBadgeText(badge: { readonly text: string }): Promise<void>;
  };
  readonly commands: {
    readonly onCommand: { addListener(listener: (name: string) => void): void };
  };
  readonly runtime: {
    readonly onMessage: {
      addListener(
        listener: (
          message: Message,
          sender: unknown,
          answer: (reply: Reply) => void,
        ) => boolean,
      ): void;
    };
    getURL(path: string): string;
    sendMessage(message: Message): Promise<Reply>;
  };
  readonly scripting: {
    executeScript<A extends ReadonlyArray<unknown>, R>(injection: {
      readonly target: { readonly tabId: number };
      readonly func: (...args: A) => R;
      readonly args: A;
    }): Promise<ReadonlyArray<{ readonly result: R }>>;
  };
  readonly storage: {
    readonly local: {
      get(key: string): Promise<Readonly<Record<string, unknown>>>;
      set(values: Readonly<Record<string, unknown>>): Promise<void>;
    };
  };
  readonly tabs: {
    captureVisibleTab(window: number, options: { readonly format: 'png' }): Promise<string>;
    create(tab: { readonly url: string; readonly index: number }): Promise<unknown>;
    getCurrent(): Promise<Tab>;
    query(filter: {
      readonly active: boolean;
      readonly lastFocusedWindow: boolean;
    }): Promise<ReadonlyArray<Tab>>;
    remove(tab: number): Promise<void>;
  };
}

export interface Tab {
  readonly id: number;
  readonly index: number;
  readonly windowId: number;
}

/** What to take a picture of. */
export type Wish = 'visible' | 'page';

/** A part of a picture, and how far down the page it is, in pixels of the page. */
export interface Piece {
  readonly url: string;
  readonly top: number;
}

/** A picture of a page, piece by piece. */
export interface Taken {
  readonly pieces: ReadonlyArray<Piece>;
  /** The size of the picture, in pixels of the page. Pieces may be wider: bars are cut off. */
  readonly width: number;
  readonly height: number;
  /** How many pixels of the picture there are in one pixel of the page. */
  readonly ratio: number;
  /** Whether the page was longer than a picture can be. */
  readonly cut: boolean;
  readonly wish: Wish;
}

/** What parts of the extension ask each other for: to take a picture, or to give it away. */
export type Message = { readonly take: Wish } | { readonly give: string };

/** What they answer. An empty reason means that there is no failure. */
export interface Reply {
  readonly reason: string;
  readonly taken: ReadonlyArray<Taken>;
}

declare global {
  var browser: Browser | undefined;
  var chrome: Browser | undefined;
}

/** Firefox has the one name, Chromium the other; newer ones have both. */
function found(): Browser {
  const known = globalThis.browser ?? globalThis.chrome;
  if (known === undefined) {
    throw new Error('This is not a browser');
  }
  return known;
}

export const browser = found();

/** Whether the browser lets extensions take no more than two pictures a second. */
export const unhurried = !browser.runtime.getURL('').startsWith('moz-extension:');

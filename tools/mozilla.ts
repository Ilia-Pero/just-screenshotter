// The catalog of Firefox: https://mozilla.github.io/addons-server/topics/api/addons.html
import { createHmac } from 'node:crypto';

/** A key to the catalog. */
export class Key {
  private readonly name: string;
  private readonly secret: string;

  constructor(name: string, secret: string) {
    this.name = name;
    this.secret = secret;
  }

  /** A pass for one request, good for two minutes after the moment, which is in seconds. */
  pass(moment: number, mark: string): string {
    const claim = [
      { alg: 'HS256', typ: 'JWT' },
      { iss: this.name, jti: mark, iat: moment, exp: moment + 120 },
    ]
      .map((part) => Buffer.from(JSON.stringify(part)).toString('base64url'))
      .join('.');
    return `${claim}.${createHmac('sha256', this.secret).update(claim).digest('base64url')}`;
  }
}

/** What carries questions to the catalog. */
export type Wire = (place: string, letter: RequestInit) => Promise<Response>;

export class Catalog {
  private readonly home: string;
  private readonly pass: () => string;
  private readonly wire: Wire;

  constructor(home: string, pass: () => string, wire: Wire) {
    this.home = home;
    this.pass = pass;
    this.wire = wire;
  }

  /** Whether the catalog has this. */
  async has(path: string): Promise<boolean> {
    const answer = await this.asked('GET', path, {});
    if (!answer.ok && answer.status !== 404) {
      throw new Error(`GET ${path}: ${answer.status} ${await answer.text()}`);
    }
    return answer.ok;
  }

  got<T>(path: string): Promise<T> {
    return this.told('GET', path, {});
  }

  /** The answer to words. */
  written<T>(method: string, path: string, words: object): Promise<T> {
    return this.told(method, path, {
      body: JSON.stringify(words),
      headers: { 'content-type': 'application/json' },
    });
  }

  /** The answer to files. */
  filed<T>(method: string, path: string, files: FormData): Promise<T> {
    return this.told(method, path, { body: files });
  }

  private async told<T>(method: string, path: string, letter: RequestInit): Promise<T> {
    const answer = await this.asked(method, path, letter);
    if (!answer.ok) {
      throw new Error(`${method} ${path}: ${answer.status} ${await answer.text()}`);
    }
    return answer.json();
  }

  private asked(method: string, path: string, letter: RequestInit): Promise<Response> {
    const headers = new Headers(letter.headers);
    headers.set('authorization', `JWT ${this.pass()}`);
    return this.wire(`${this.home}${path}`, { ...letter, method, headers });
  }
}

/** What people read about the extension in the catalog. */
export interface Listing {
  readonly id: string;
  readonly license: string;
  /** What the people who check it should know. */
  readonly notes: string;
  readonly about: Readonly<Record<string, unknown>>;
}

/** A version of the extension and what comes with it. */
export interface Parcel {
  readonly version: string;
  readonly news: string;
  readonly pack: File;
  readonly source: File;
  readonly logo: File;
  readonly screen: File;
}

interface Upload {
  readonly uuid: string;
  readonly processed: boolean;
  readonly valid: boolean;
  readonly validation: unknown;
}

interface Entry {
  readonly url: string;
  readonly previews: ReadonlyArray<unknown>;
}

/** The extension in the catalog. */
export class Extension {
  private readonly catalog: Catalog;
  private readonly listing: Listing;
  private readonly pause: () => Promise<void>;

  constructor(catalog: Catalog, listing: Listing, pause: () => Promise<void>) {
    this.catalog = catalog;
    this.listing = listing;
    this.pause = pause;
  }

  /** Sends what the catalog does not have yet and says where the extension is seen. */
  async send(parcel: Parcel): Promise<string> {
    const home = `addons/addon/${this.listing.id}/`;
    const place = `${home}versions/v${parcel.version}/`;
    if (!(await this.catalog.has(place))) {
      const upload = await this.checked(parcel.pack);
      await this.catalog.written('PUT', home, this.words(parcel, upload));
    }
    await this.catalog.filed('PATCH', place, form({ source: parcel.source }));
    const entry = await this.catalog.got<Entry>(home);
    if (entry.previews.length === 0) {
      await this.catalog.filed('PATCH', home, form({ icon: parcel.logo }));
      await this.catalog.filed('POST', `${home}previews/`, form({ image: parcel.screen }));
    }
    return entry.url;
  }

  private words(parcel: Parcel, upload: string): object {
    return {
      ...this.listing.about,
      version: {
        upload,
        license: this.listing.license,
        compatibility: ['firefox'],
        approval_notes: this.listing.notes,
        release_notes: { 'en-US': parcel.news },
      },
    };
  }

  /** The number of the upload, once the catalog has looked through it and found nothing wrong. */
  private async checked(pack: File): Promise<string> {
    const files = form({ upload: pack });
    files.set('channel', 'listed');
    const upload = await this.settled(await this.catalog.filed('POST', 'addons/upload/', files));
    if (!upload.valid) {
      throw new Error(`The package is not valid: ${JSON.stringify(upload.validation)}`);
    }
    return upload.uuid;
  }

  private async settled(upload: Upload): Promise<Upload> {
    if (upload.processed) {
      return upload;
    }
    await this.pause();
    return this.settled(await this.catalog.got(`addons/upload/${upload.uuid}/`));
  }
}

function form(files: Readonly<Record<string, File>>): FormData {
  const data = new FormData();
  Object.entries(files).forEach(([name, file]) => data.set(name, file));
  return data;
}

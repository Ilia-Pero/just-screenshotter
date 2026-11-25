import { Box } from './box.ts';
import type { Desk, Ears, Opening, Tongue } from './desk.ts';
import { LANGUAGES, spoken } from './languages.ts';
import { developed } from './photo.ts';
import { Point } from './point.ts';
import type { Shot } from './shot.ts';
import type { Words } from './words.ts';

/** A pretended program window on the sample screenshot. Sizes are fractions of the screen. */
interface Pane {
  readonly left: number;
  readonly top: number;
  readonly width: number;
  readonly height: number;
  readonly title: string;
  readonly tint: string;
}

const PANES: ReadonlyArray<Pane> = [
  { left: 0.06, top: 0.08, width: 0.52, height: 0.62, title: 'Quarterly report', tint: '#33415c' },
  { left: 0.46, top: 0.28, width: 0.47, height: 0.56, title: 'Notes', tint: '#5c4d7d' },
];

/**
 * A browser tab instead of the desktop. It lets the editor run without the shell:
 * the screen is replaced by a sample picture, and the result is shown on the page.
 */
export class Sandbox implements Desk {
  private readonly root: HTMLElement;

  constructor(root: HTMLElement) {
    this.root = root;
  }

  shot(): Promise<Shot> {
    const ratio = window.devicePixelRatio;
    const canvas = document.createElement('canvas');
    canvas.width = Math.round(this.root.clientWidth * ratio);
    canvas.height = Math.round(this.root.clientHeight * ratio);
    const ctx = canvas.getContext('2d');
    if (ctx === null) {
      return Promise.reject(new Error('The browser has no canvas for the sample picture'));
    }
    const sky = ctx.createLinearGradient(0, 0, canvas.width, canvas.height);
    sky.addColorStop(0, '#1d4e89');
    sky.addColorStop(1, '#7fb7be');
    ctx.fillStyle = sky;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    PANES.forEach((pane) => this.pane(ctx, pane, ratio));
    return developed(canvas.toDataURL('image/png'));
  }

  opening(): Promise<Opening> {
    const corner = new Point(0, 0);
    return Promise.resolve({ area: new Box(corner, corner), film: false });
  }

  copy(png: Blob): Promise<void> {
    return Promise.resolve()
      .then(() => navigator.clipboard.write([new ClipboardItem({ 'image/png': png })]))
      .then(
        () => 'The picture is on the clipboard.',
        () => 'The browser keeps the clipboard closed. Here is the picture itself.',
      )
      .then((note) => this.card(note, png));
  }

  save(png: Blob): Promise<void> {
    const link = document.createElement('a');
    link.href = URL.createObjectURL(png);
    link.download = 'Screenshot.png';
    link.click();
    this.card('The picture went to the downloads of the browser.', png);
    return Promise.resolve();
  }

  shares(): Promise<boolean> {
    return Promise.resolve(true);
  }

  share(png: Blob): Promise<void> {
    this.card('The picture would go to the server, and the link to the clipboard.', png);
    return Promise.resolve();
  }

  films(): Promise<boolean> {
    return Promise.resolve(true);
  }

  listens(): Promise<Ears> {
    return Promise.resolve({ hears: true, sound: true, voice: false });
  }

  film(area: Box): Promise<void> {
    const note = document.createElement('div');
    note.className = 'card';
    note.textContent = `A film of ${area.width()} by ${area.height()} would be made.`;
    this.root.appendChild(note);
    return this.leave();
  }

  languages(): Promise<ReadonlyArray<Tongue>> {
    return Promise.resolve(
      Object.keys(LANGUAGES).map((code) => ({ code, title: spoken(code).language, on: false })),
    );
  }

  speak(code: string): Promise<Words> {
    return Promise.resolve(spoken(code));
  }

  leave(): Promise<void> {
    const card = document.createElement('div');
    const again = document.createElement('button');
    card.className = 'card';
    card.textContent = 'The editor is closed. ';
    again.type = 'button';
    again.textContent = 'Open again';
    again.addEventListener('click', () => window.location.reload());
    card.appendChild(again);
    this.root.appendChild(card);
    this.root.classList.add('closed');
    return Promise.resolve();
  }

  /** Shows a message and the resulting picture on the page. */
  private card(note: string, png: Blob): void {
    const card = document.createElement('div');
    const text = document.createElement('p');
    const image = document.createElement('img');
    card.className = 'card';
    text.textContent = note;
    image.src = URL.createObjectURL(png);
    image.alt = 'The resulting picture';
    card.appendChild(text);
    card.appendChild(image);
    this.root.appendChild(card);
  }

  private pane(ctx: CanvasRenderingContext2D, pane: Pane, ratio: number): void {
    const x = pane.left * ctx.canvas.width;
    const y = pane.top * ctx.canvas.height;
    const width = pane.width * ctx.canvas.width;
    const height = pane.height * ctx.canvas.height;
    ctx.fillStyle = '#f7f7f7';
    ctx.fillRect(x, y, width, height);
    ctx.fillStyle = pane.tint;
    ctx.fillRect(x, y, width, 30 * ratio);
    ctx.fillStyle = '#ffffff';
    ctx.font = `${14 * ratio}px sans-serif`;
    ctx.textBaseline = 'middle';
    ctx.fillText(pane.title, x + 12 * ratio, y + 15 * ratio);
    ctx.fillStyle = '#c9ccd6';
    for (let line = 0; line * 26 * ratio < height - 80 * ratio; line += 1) {
      ctx.fillRect(
        x + 16 * ratio,
        y + (52 + line * 26) * ratio,
        width * (0.9 - (line % 3) * 0.2),
        10 * ratio,
      );
    }
  }
}

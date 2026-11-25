import { Box } from './box.ts';
import type { Bridge } from './bridge.ts';
import type { Desk, Ears, Opening } from './desk.ts';
import { developed } from './photo.ts';
import { Point } from './point.ts';
import type { Shot } from './shot.ts';

/** How many pixels may be lost when the window is measured. */
const CLOSE = 3;

/** The opening as the shell tells it. */
interface Told {
  readonly left: number;
  readonly top: number;
  readonly width: number;
  readonly height: number;
  readonly film: boolean;
}

/** The desktop of the real program: the shell written in Rust owns the screen and the clipboard. */
export class Shell implements Desk {
  private readonly bridge: Bridge;

  constructor(bridge: Bridge) {
    this.bridge = bridge;
  }

  shot(): Promise<Shot> {
    return this.bridge
      .invoke<ArrayBuffer>('shot')
      .then((bytes) => developed(URL.createObjectURL(new Blob([bytes], { type: 'image/png' }))));
  }

  opening(): Promise<Opening> {
    return this.bridge.invoke<Told>('opening').then((told) => ({
      area: new Box(
        new Point(told.left, told.top),
        new Point(told.left + told.width, told.top + told.height),
      ),
      film: told.film,
    }));
  }

  /**
   * Waits until the window covers the screen. Linux stretches only windows that are shown, so such
   * a window is shown before the picture is drawn. If it does not cover the screen in a second,
   * it never will.
   */
  stretched(width: number): Promise<void> {
    const room = (): number =>
      Math.abs(document.documentElement.clientWidth * window.devicePixelRatio - width);
    if (room() < CLOSE) {
      return Promise.resolve();
    }
    return this.bridge.invoke<void>('reveal').then(
      () =>
        new Promise((resolve) => {
          const look = (): void => {
            if (room() < CLOSE) {
              done();
            }
          };
          const done = (): void => {
            window.removeEventListener('resize', look);
            resolve();
          };
          window.addEventListener('resize', look);
          window.setTimeout(done, 1000);
          look();
        }),
    );
  }

  copy(png: Blob): Promise<void> {
    return this.bytes(png).then((data) => this.bridge.invoke<void>('copy', data));
  }

  save(png: Blob): Promise<void> {
    return this.bytes(png).then((data) => this.bridge.invoke<void>('save', data));
  }

  shares(): Promise<boolean> {
    return this.bridge.invoke<boolean>('shares');
  }

  share(png: Blob): Promise<void> {
    return this.bytes(png).then((data) => this.bridge.invoke<void>('share', data));
  }

  films(): Promise<boolean> {
    return Promise.resolve(true);
  }

  listens(): Promise<Ears> {
    return this.bridge.invoke<Ears>('listens');
  }

  film(area: Box, sheet: Box, ears: Ears): Promise<void> {
    return this.bridge.invoke<void>('film', {
      wish: {
        area: {
          left: Math.round(area.left()),
          top: Math.round(area.top()),
          width: Math.round(area.width()),
          height: Math.round(area.height()),
        },
        sheet: [sheet.width(), sheet.height()],
        sound: ears.sound,
        voice: ears.voice,
      },
    });
  }

  leave(): Promise<void> {
    return this.bridge.invoke<void>('leave');
  }

  /** The picture as bytes. Through Response, because Safari 11 has no Blob.arrayBuffer. */
  private bytes(png: Blob): Promise<Uint8Array> {
    return new Response(png).arrayBuffer().then((buffer) => new Uint8Array(buffer));
  }
}

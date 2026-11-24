import type { Bridge } from './bridge.ts';
import type { Desk } from './desk.ts';
import { developed } from './photo.ts';
import type { Shot } from './shot.ts';

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

  leave(): Promise<void> {
    return this.bridge.invoke<void>('leave');
  }

  /** The picture as bytes. Through Response, because Safari 11 has no Blob.arrayBuffer. */
  private bytes(png: Blob): Promise<Uint8Array> {
    return new Response(png).arrayBuffer().then((buffer) => new Uint8Array(buffer));
  }
}

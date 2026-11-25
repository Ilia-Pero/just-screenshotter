import './bridge.ts';
import type { Bridge } from './bridge.ts';
import type { Cabinet, Chosen, State } from './cabinet.ts';
import type { Tongue } from './desk.ts';
import { Room } from './room.ts';
import type { Words } from './words.ts';

/** The settings the shell keeps. */
class Shelf implements Cabinet {
  private readonly bridge: Bridge;

  constructor(bridge: Bridge) {
    this.bridge = bridge;
  }

  words(): Promise<Words> {
    return this.bridge.invoke<Words>('words');
  }

  state(): Promise<State> {
    return this.bridge.invoke<State>('panel');
  }

  arrange(chosen: ReadonlyArray<Chosen>): Promise<State> {
    return this.bridge.invoke<State>('arrange', { chosen });
  }

  hold(on: boolean): Promise<void> {
    return this.bridge.invoke<void>('hold', { on });
  }

  launch(on: boolean): Promise<State> {
    return this.bridge.invoke<State>('launch', { on });
  }

  hear(sound: boolean, voice: boolean): Promise<State> {
    return this.bridge.invoke<State>('hear', { sound, voice });
  }

  relocate(): Promise<State> {
    return this.bridge.invoke<State>('relocate');
  }

  folder(): Promise<void> {
    return this.bridge.invoke<void>('folder');
  }

  uploading(): Promise<void> {
    return this.bridge.invoke<void>('uploading');
  }

  languages(): Promise<ReadonlyArray<Tongue>> {
    return this.bridge.invoke<ReadonlyArray<Tongue>>('languages');
  }

  speak(code: string): Promise<Words> {
    return this.bridge.invoke<Words>('speak', { code });
  }
}

new Room(document.body, new Shelf(window.__TAURI_INTERNALS__)).open();

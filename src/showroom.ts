import type { Cabinet, Chosen, State } from './cabinet.ts';
import type { Tongue } from './desk.ts';
import { LANGUAGES, spoken } from './languages.ts';
import { Room } from './room.ts';
import type { Words } from './words.ts';

/** Settings that nobody keeps: a browser tab for a look at the window. */
class Pretence implements Cabinet {
  private code: string;
  private now: State = {
    acts: [
      { code: 'area', key: 'PrintScreen', title: 'Print Screen', mouse: 'both', busy: false },
      {
        code: 'screen',
        key: 'Shift+PrintScreen',
        title: 'Shift + Print Screen',
        mouse: '',
        busy: false,
      },
      { code: 'window', key: 'Alt+PrintScreen', title: 'Alt + Print Screen', mouse: '', busy: true },
      { code: 'film', key: '', title: '', mouse: 'shift+both', busy: false },
    ],
    assignable: true,
    mice: true,
    folder: 'C:\\Users\\Person\\Pictures\\Screenshots',
    ears: { hears: true, sound: true, voice: false },
    launch: true,
    version: '0.0.0',
  };

  constructor(code: string) {
    this.code = code;
  }

  words(): Promise<Words> {
    return Promise.resolve(spoken(this.code));
  }

  state(): Promise<State> {
    return Promise.resolve(this.now);
  }

  arrange(chosen: ReadonlyArray<Chosen>): Promise<State> {
    return this.changed({
      acts: this.now.acts.map((row, index) =>
        Object.assign({}, row, chosen[index], {
          title: (chosen[index]?.key ?? '').split('+').join(' + '),
        }),
      ),
    });
  }

  hold(): Promise<void> {
    return Promise.resolve();
  }

  launch(on: boolean): Promise<State> {
    return this.changed({ launch: on });
  }

  hear(sound: boolean, voice: boolean): Promise<State> {
    return this.changed({ ears: { hears: true, sound, voice } });
  }

  relocate(): Promise<State> {
    return this.changed({ folder: 'D:\\Screenshots' });
  }

  folder(): Promise<void> {
    return Promise.reject(new Error('A browser tab has no folders.'));
  }

  uploading(): Promise<void> {
    return Promise.reject(new Error('A browser tab has no server.'));
  }

  languages(): Promise<ReadonlyArray<Tongue>> {
    return Promise.resolve(
      Object.keys(LANGUAGES).map((code) => ({
        code,
        title: spoken(code).language,
        on: spoken(code) === spoken(this.code),
      })),
    );
  }

  speak(code: string): Promise<Words> {
    this.code = code;
    return this.words();
  }

  private changed(change: Partial<State>): Promise<State> {
    this.now = Object.assign({}, this.now, change);
    return this.state();
  }
}

// The language may be given in the address: http://127.0.0.1:5391/panel.html?he
new Room(
  document.body,
  new Pretence(window.location.search.slice(1) || navigator.language),
).open();

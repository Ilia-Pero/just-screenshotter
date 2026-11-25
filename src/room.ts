import type { Cabinet, Row, State } from './cabinet.ts';
import { chosen } from './cabinet.ts';
import { Chord } from './chord.ts';
import type { Tongue } from './desk.ts';
import type { Words } from './words.ts';

/** Names of actions. */
const ACTS: Readonly<Record<string, keyof Words>> = {
  area: 'act.area',
  screen: 'act.screen',
  window: 'act.window',
  film: 'act.film',
};

/** Names of what the mouse can do. */
const MICE: ReadonlyArray<readonly [string, keyof Words]> = [
  ['', 'settings.none'],
  ['both', 'mouse.both'],
  ['shift+both', 'mouse.shift'],
  ['ctrl+both', 'mouse.control'],
];

function made<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  name: string,
  text: string,
): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  node.className = name;
  node.textContent = text;
  return node;
}

/** The element with the children inside. */
function holding<T extends HTMLElement>(node: T, ...children: ReadonlyArray<HTMLElement>): T {
  children.forEach((child) => node.appendChild(child));
  return node;
}

/**
 * The window with the settings. Everything a person changes is in force at once, so there is
 * nothing to save. The window is drawn anew after every change.
 */
export class Room {
  private readonly root: HTMLElement;
  private readonly cabinet: Cabinet;
  private readonly alarm = made('p', 'bad hidden', '');
  /** Ends listening to the keyboard. There is nothing to end while nobody listens. */
  private hush: () => void = () => undefined;

  constructor(root: HTMLElement, cabinet: Cabinet) {
    this.root = root;
    this.cabinet = cabinet;
  }

  open(): void {
    window.addEventListener('blur', () => this.hush());
    this.cabinet
      .words()
      .then((words) => this.enter(words))
      .catch((error: unknown) => this.complain(error));
  }

  private enter(words: Words): Promise<void> {
    return Promise.all([this.cabinet.state(), this.cabinet.languages()]).then((ready) =>
      this.draw(words, ready[0], ready[1]),
    );
  }

  private draw(words: Words, state: State, tongues: ReadonlyArray<Tongue>): void {
    document.title = words.settings;
    this.root.dir = words.direction;
    this.root.textContent = '';
    this.alarm.classList.add('hidden');
    const show = (after: Promise<State>): void => {
      after
        .then((fresh) => this.draw(words, fresh, tongues))
        .catch((error: unknown) => {
          this.draw(words, state, tongues);
          this.complain(error);
        });
    };
    holding(
      this.root,
      this.alarm,
      made('h2', '', words['settings.start']),
      ...this.keys(words, state, show),
      made('h2', '', words['settings.film']),
      this.tick(words['film.sound'], state.ears.sound, state.ears.hears, (on) =>
        show(this.cabinet.hear(on, state.ears.voice)),
      ),
      this.tick(words['film.voice'], state.ears.voice, true, (on) =>
        show(this.cabinet.hear(state.ears.sound, on)),
      ),
      made('h2', '', words['settings.folder']),
      made('span', 'path', state.folder),
      holding(
        made('div', 'buttons', ''),
        this.button(words['settings.change'], () => show(this.cabinet.relocate())),
        this.button(words['menu.folder'], () => this.run(this.cabinet.folder())),
        this.button(words['menu.server'], () => this.run(this.cabinet.uploading())),
      ),
      made('h2', '', words['menu.language']),
      this.tongue(tongues),
      made('h2', '', words['settings.program']),
      this.tick(words['menu.autostart'], state.launch, true, (on) =>
        show(this.cabinet.launch(on)),
      ),
      made('p', 'note', `${words.name} ${state.version}`),
    );
  }

  /** The table of actions, or an explanation why there is none. */
  private keys(
    words: Words,
    state: State,
    show: (after: Promise<State>) => void,
  ): ReadonlyArray<HTMLElement> {
    if (!state.assignable) {
      return [made('p', '', words['error.shortcut'])];
    }
    const table = made('table', '', '');
    const head = holding(
      made('tr', '', ''),
      made('th', '', ''),
      made('th', '', words['settings.keys']),
    );
    if (state.mice) {
      head.appendChild(made('th', '', words['settings.mouse']));
    }
    table.appendChild(head);
    state.acts.forEach((row) => {
      const line = holding(
        made('tr', '', ''),
        made('td', '', words[ACTS[row.code] ?? 'act.area']),
        holding(made('td', '', ''), ...this.key(words, state, row, show)),
      );
      if (state.mice) {
        line.appendChild(holding(made('td', '', ''), this.mouse(words, state, row, show)));
      }
      table.appendChild(line);
    });
    const busy = state.acts.some((row) => row.busy);
    return [
      table,
      made('p', busy ? 'bad' : 'hidden', words['settings.busy']),
      made('p', state.mice ? 'note' : 'hidden', words['settings.cost']),
    ];
  }

  /** The button that waits for a key, and the one that leaves the action without a key. */
  private key(
    words: Words,
    state: State,
    row: Row,
    show: (after: Promise<State>) => void,
  ): ReadonlyArray<HTMLElement> {
    const take = (key: string): void =>
      show(this.cabinet.arrange(chosen(state.acts, row.code, { key })));
    const button = this.button(row.title === '' ? words['settings.none'] : row.title, () =>
      this.listen(button, words, take, () => show(this.cabinet.state())),
    );
    const clear = this.button('✕', () => take(''));
    button.classList.add('key');
    button.classList.toggle('empty', row.key === '');
    button.classList.toggle('busy', row.busy);
    clear.className = row.key === '' ? 'hidden' : 'clear';
    clear.title = words['settings.clear'];
    clear.setAttribute('aria-label', words['settings.clear']);
    return [button, clear];
  }

  /** Waits for keys. Escape, or a look at another window, leaves everything as it was. */
  private listen(
    button: HTMLButtonElement,
    words: Words,
    take: (key: string) => void,
    leave: () => void,
  ): void {
    this.hush();
    const struck = (event: KeyboardEvent): void => {
      const chord = new Chord(event);
      event.preventDefault();
      event.stopPropagation();
      if (event.code === 'Escape') {
        this.hush();
      } else if (chord.fit()) {
        this.deafen();
        take(chord.spelled());
      } else if (!chord.pending()) {
        button.textContent = words['settings.unfit'];
      }
    };
    // Windows tells about Print Screen only when it is released.
    const released = (event: KeyboardEvent): void => {
      if (event.code === 'PrintScreen' || event.key === 'PrintScreen') {
        struck(event);
      }
    };
    window.addEventListener('keydown', struck, true);
    window.addEventListener('keyup', released, true);
    button.classList.add('listening');
    button.textContent = words['settings.press'];
    this.deafen = () => {
      window.removeEventListener('keydown', struck, true);
      window.removeEventListener('keyup', released, true);
      this.hush = () => undefined;
    };
    this.hush = () => {
      this.deafen();
      this.run(this.cabinet.hold(false));
      leave();
    };
    this.run(this.cabinet.hold(true));
  }

  /** Stops hearing keys. */
  private deafen: () => void = () => undefined;

  private mouse(
    words: Words,
    state: State,
    row: Row,
    show: (after: Promise<State>) => void,
  ): HTMLElement {
    const select = this.select(
      MICE.map(([code, title]) => [code, words[title]]),
      row.mouse,
      (mouse) => show(this.cabinet.arrange(chosen(state.acts, row.code, { mouse }))),
    );
    select.setAttribute('aria-label', words['settings.mouse']);
    return select;
  }

  private tongue(tongues: ReadonlyArray<Tongue>): HTMLElement {
    const on = tongues.filter((tongue) => tongue.on).map((tongue) => tongue.code);
    const select = this.select(
      tongues.map((tongue) => [tongue.code, tongue.title]),
      on[0] ?? '',
      (code) => {
        this.cabinet
          .speak(code)
          .then((words) => this.enter(words))
          .catch((error: unknown) => this.complain(error));
      },
    );
    select.className = 'tongue';
    return select;
  }

  private select(
    options: ReadonlyArray<readonly [string, string]>,
    value: string,
    choose: (value: string) => void,
  ): HTMLSelectElement {
    const select = made('select', '', '');
    options.forEach(([code, title]) => {
      const option = made('option', '', title);
      option.value = code;
      select.appendChild(option);
    });
    select.value = value;
    select.addEventListener('change', () => choose(select.value));
    return select;
  }

  private tick(
    title: string,
    on: boolean,
    possible: boolean,
    change: (on: boolean) => void,
  ): HTMLElement {
    const box = made('input', '', '');
    box.type = 'checkbox';
    box.checked = on;
    box.disabled = !possible;
    box.addEventListener('change', () => change(box.checked));
    return holding(made('label', 'row', ''), box, made('span', '', title));
  }

  private button(title: string, press: () => void): HTMLButtonElement {
    const button = made('button', '', title);
    button.type = 'button';
    button.addEventListener('click', press);
    return button;
  }

  private run(work: Promise<void>): void {
    work.catch((error: unknown) => this.complain(error));
  }

  private complain(error: unknown): void {
    this.alarm.textContent = String(error);
    this.alarm.classList.remove('hidden');
    if (this.alarm.parentNode === null) {
      this.root.insertBefore(this.alarm, this.root.firstChild);
    }
    this.alarm.scrollIntoView();
  }
}

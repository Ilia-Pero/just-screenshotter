import type { Desk, Ears, Tongue } from './desk.ts';
import type { Gesture } from './gesture.ts';
import { Rest } from './gesture.ts';
import {
  BACKSPACE,
  DELETE,
  ENTER,
  ESCAPE,
  LETTER_C,
  LETTER_S,
  LETTER_U,
  LETTER_Z,
  PAD_ENTER,
} from './key.ts';
import type { Acts, Panels } from './panels.ts';
import { Point } from './point.ts';
import type { Session } from './session.ts';
import type { Typing } from './shape.ts';
import { turned } from './thickness.ts';
import type { Tool } from './tool.ts';

/** How far the wheel has to go to change the thickness once, in pixels. */
const NOTCH = 50;

/**
 * The editor: the screenshot on the canvas, the selection and the panels.
 * This is the only place where state changes: the current movement of the mouse and the mode.
 * unit is the number of screenshot pixels in one pixel of the window.
 */
export class Overlay implements Acts {
  private readonly desk: Desk;
  private readonly ctx: CanvasRenderingContext2D;
  private readonly unit: number;
  private readonly panels: Panels;
  private readonly typing: Typing;
  private gesture: Gesture;
  /** Whether a film is wanted and has not been proposed yet. */
  private wanted: boolean;
  private mode: 'idle' | 'dragging' | 'busy' = 'idle';
  private queued = false;
  private rolled = 0;

  constructor(
    desk: Desk,
    ctx: CanvasRenderingContext2D,
    unit: number,
    panels: Panels,
    typing: Typing,
    gesture: Gesture,
    wanted: boolean,
  ) {
    this.desk = desk;
    this.ctx = ctx;
    this.unit = unit;
    this.panels = panels;
    this.typing = typing;
    this.gesture = gesture;
    this.wanted = wanted;
  }

  open(): void {
    const canvas = this.ctx.canvas;
    this.panels.build(this);
    canvas.addEventListener('mousedown', (event) => this.press(event));
    canvas.addEventListener('dblclick', (event) => this.reword(event));
    canvas.addEventListener('wheel', (event) => this.roll(event));
    window.addEventListener('mousemove', (event) => this.drag(event));
    window.addEventListener('mouseup', (event) => this.release(event));
    window.addEventListener('keydown', (event) => this.strike(event));
    window.addEventListener('resize', () => this.refresh());
    window.addEventListener('scroll', () => this.refresh());
    window.addEventListener('contextmenu', (event) => event.preventDefault());
    this.paint();
    this.propose();
  }

  copy(): void {
    this.send(false, (png) => this.desk.copy(png));
  }

  save(): void {
    this.send(false, (png) => this.desk.save(png));
  }

  share(): void {
    this.send(true, (png) => this.desk.share(png));
  }

  leave(): void {
    this.desk.leave().catch((error: unknown) => this.panels.complain(error));
  }

  undo(): void {
    this.alter((session) => session.undone());
  }

  drop(): void {
    this.alter((session) => session.dropped());
  }

  equip(tool: Tool): void {
    this.alter((session) => session.equipped(tool));
  }

  dye(color: string): void {
    this.alter((session) => session.dyed(color));
  }

  thick(width: number): void {
    this.alter((session) => session.thick(width));
  }

  listens(): Promise<Ears> {
    return this.desk.listens();
  }

  film(ears: Ears): void {
    const session = this.gesture.now();
    if (this.mode !== 'idle' || session.place().blank()) {
      return;
    }
    this.mode = 'busy';
    this.desk.film(session.place(), session.sheet(), ears).catch((error: unknown) => {
      this.mode = 'idle';
      this.panels.complain(error);
    });
  }

  languages(): Promise<ReadonlyArray<Tongue>> {
    return this.desk.languages();
  }

  speak(code: string): void {
    this.desk
      .speak(code)
      .then((words) => {
        this.panels.speak(words);
        this.paint();
      })
      .catch((error: unknown) => this.panels.complain(error));
  }

  private press(event: MouseEvent): void {
    if (event.button !== 0 || this.mode !== 'idle') {
      return;
    }
    event.preventDefault();
    this.panels.hush();
    this.mode = 'dragging';
    this.gesture = this.gesture.now().press(this.spot(event));
    this.paint();
  }

  private drag(event: MouseEvent): void {
    if (this.mode === 'dragging') {
      this.gesture = this.gesture.moved(this.spot(event));
      this.refresh();
    } else if (this.mode === 'idle') {
      this.ctx.canvas.style.cursor = this.gesture.now().cursor(this.spot(event));
    }
  }

  private release(event: MouseEvent): void {
    if (event.button === 0 && this.mode === 'dragging') {
      this.end(this.gesture.moved(this.spot(event)));
    }
  }

  /** A double click on a text starts typing it anew. */
  private reword(event: MouseEvent): void {
    if (this.mode === 'idle') {
      this.end(this.gesture.now().twice(this.spot(event)));
    }
  }

  /** The wheel changes the thickness. */
  private roll(event: WheelEvent): void {
    event.preventDefault();
    this.rolled += event.deltaY;
    const notches = Math.trunc(this.rolled / NOTCH);
    this.rolled -= notches * NOTCH;
    const width = this.gesture.now().thickness() / this.unit;
    if (notches !== 0 && width > 0) {
      this.thick(turned(width, -notches) * this.unit);
    }
  }

  private strike(event: KeyboardEvent): void {
    const held = event.ctrlKey || event.metaKey;
    if (ESCAPE.struck(event)) {
      this.leave();
    } else if (held && LETTER_C.struck(event)) {
      this.copy();
    } else if (held && LETTER_S.struck(event)) {
      this.save();
    } else if (held && LETTER_U.struck(event)) {
      this.share();
    } else if (held && LETTER_Z.struck(event)) {
      this.undo();
    } else if (DELETE.struck(event) || BACKSPACE.struck(event)) {
      this.drop();
    } else if (
      this.mode === 'idle' &&
      event.target === document.body &&
      (ENTER.struck(event) || PAD_ENTER.struck(event))
    ) {
      // With the focus on a button, Enter presses the button.
      this.end(this.gesture.now().rewrite());
    } else {
      return;
    }
    event.preventDefault();
  }

  /** Changes the session, unless something else is going on with it. */
  private alter(change: (session: Session) => Session): void {
    if (this.mode === 'idle') {
      this.rest(change(this.gesture.now()));
    }
  }

  /** Waits for what the movement leads to: a person may have to type a text. */
  private end(gesture: Gesture): void {
    const before = this.gesture.now();
    this.mode = 'busy';
    this.gesture = gesture;
    // The panels have to be within reach again: a click on them also ends typing.
    this.paint();
    gesture
      .ended(this.typing)
      .catch((error: unknown) => {
        this.panels.complain(error);
        return before;
      })
      .then((session) => this.rest(session))
      .catch((error: unknown) => this.panels.complain(error));
  }

  private rest(session: Session): void {
    this.gesture = new Rest(session);
    this.mode = 'idle';
    this.paint();
    this.propose();
  }

  /** Proposes to film as soon as there is an area, if a person has come for that. */
  private propose(): void {
    if (this.wanted && !this.gesture.now().place().blank()) {
      this.wanted = false;
      this.panels.propose();
    }
  }

  /** Hands the selected part over and closes the editor. After a failure the editor stays. */
  private send(long: boolean, deliver: (png: Blob) => Promise<void>): void {
    const session = this.gesture.now();
    if (this.mode !== 'idle' || session.place().blank()) {
      return;
    }
    this.mode = 'busy';
    this.panels.announce(long);
    session
      .cut()
      .png()
      .then(deliver)
      .then(() => this.desk.leave())
      .catch((error: unknown) => {
        this.mode = 'idle';
        this.panels.complain(error);
      });
  }

  /**
   * Repaints at once, without waiting for the next frame. Otherwise a click right after a drag
   * would miss a button that has not moved to its new place yet.
   */
  private paint(): void {
    this.queued = false;
    this.gesture.paint(this.ctx);
    this.panels.show(this.gesture.now(), this.mode === 'dragging');
  }

  /** Repaints on the next frame: while the mouse moves, there is no need to do it more often. */
  private refresh(): void {
    if (!this.queued) {
      this.queued = true;
      window.requestAnimationFrame(() => this.paint());
    }
  }

  private spot(event: MouseEvent): Point {
    return new Point(
      Math.round(event.pageX * this.unit),
      Math.round(event.pageY * this.unit),
    );
  }
}

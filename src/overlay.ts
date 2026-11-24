import type { Desk } from './desk.ts';
import type { Gesture } from './gesture.ts';
import { Rest } from './gesture.ts';
import { ESCAPE, LETTER_C, LETTER_S, LETTER_U, LETTER_Z } from './key.ts';
import type { Acts, Panels } from './panels.ts';
import { Point } from './point.ts';
import type { Session } from './session.ts';
import type { Typing } from './shape.ts';
import type { Tool } from './tool.ts';
import type { Words } from './words.ts';

/**
 * The editor: the screenshot on the canvas, the selection and the panels.
 * This is the only place where state changes: the current movement of the mouse and the mode.
 * unit is the number of screenshot pixels in one pixel of the window.
 */
export class Overlay implements Acts {
  private readonly desk: Desk;
  private readonly words: Words;
  private readonly ctx: CanvasRenderingContext2D;
  private readonly unit: number;
  private readonly panels: Panels;
  private readonly typing: Typing;
  private gesture: Gesture;
  private mode: 'idle' | 'dragging' | 'busy' = 'idle';
  private queued = false;

  constructor(
    desk: Desk,
    words: Words,
    ctx: CanvasRenderingContext2D,
    unit: number,
    panels: Panels,
    typing: Typing,
    gesture: Gesture,
  ) {
    this.desk = desk;
    this.words = words;
    this.ctx = ctx;
    this.unit = unit;
    this.panels = panels;
    this.typing = typing;
    this.gesture = gesture;
  }

  open(): void {
    this.panels.build(this);
    this.ctx.canvas.addEventListener('mousedown', (event) => this.press(event));
    window.addEventListener('mousemove', (event) => this.drag(event));
    window.addEventListener('mouseup', (event) => this.release(event));
    window.addEventListener('keydown', (event) => this.strike(event));
    window.addEventListener('resize', () => this.refresh());
    window.addEventListener('contextmenu', (event) => event.preventDefault());
    this.paint();
  }

  copy(): void {
    this.send('', (png) => this.desk.copy(png));
  }

  save(): void {
    this.send('', (png) => this.desk.save(png));
  }

  share(): void {
    this.send(this.words.sharing, (png) => this.desk.share(png));
  }

  leave(): void {
    this.desk.leave().catch((error: unknown) => this.panels.complain(error));
  }

  undo(): void {
    this.alter((session) => session.undone());
  }

  equip(tool: Tool): void {
    this.alter((session) => session.equipped(tool));
  }

  dye(color: string): void {
    this.alter((session) => session.dyed(color));
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
    if (event.button !== 0 || this.mode !== 'dragging') {
      return;
    }
    const before = this.gesture.now();
    this.mode = 'busy';
    this.gesture = this.gesture.moved(this.spot(event));
    // The panels have to be within reach again: a click on them also ends typing.
    this.paint();
    this.gesture
      .ended(this.typing)
      .catch((error: unknown) => {
        this.panels.complain(error);
        return before;
      })
      .then((session) => this.rest(session))
      .catch((error: unknown) => this.panels.complain(error));
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

  private rest(session: Session): void {
    this.gesture = new Rest(session);
    this.mode = 'idle';
    this.paint();
  }

  /** Hands the selected part over and closes the editor. After a failure the editor stays. */
  private send(doing: string, deliver: (png: Blob) => Promise<void>): void {
    const session = this.gesture.now();
    if (this.mode !== 'idle' || session.place().blank()) {
      return;
    }
    this.mode = 'busy';
    this.panels.announce(doing);
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
      Math.round(event.clientX * this.unit),
      Math.round(event.clientY * this.unit),
    );
  }
}

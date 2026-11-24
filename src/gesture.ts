import { Box } from './box.ts';
import type { Grip } from './grip.ts';
import type { Point } from './point.ts';
import type { Session } from './session.ts';
import type { Draft, Typing } from './shape.ts';
import { Sketch } from './sketch.ts';

/** One movement of the mouse, from pressing the button to releasing it. */
export interface Gesture {
  /** The same movement, continued to the point. */
  moved(to: Point): Gesture;
  /** The session as it looks in the middle of the movement. */
  now(): Session;
  paint(ctx: CanvasRenderingContext2D): void;
  /** The session after the button is released. */
  ended(typing: Typing): Promise<Session>;
}

/** The mouse button is up. */
export class Rest implements Gesture {
  private readonly session: Session;

  constructor(session: Session) {
    this.session = session;
  }

  moved(): Gesture {
    return this;
  }

  now(): Session {
    return this.session;
  }

  paint(ctx: CanvasRenderingContext2D): void {
    this.session.paint(ctx, new Sketch([]));
  }

  ended(): Promise<Session> {
    return Promise.resolve(this.session);
  }
}

/** Dragging out a new selection. */
export class Framing implements Gesture {
  private readonly session: Session;
  private readonly from: Point;
  private readonly to: Point;

  constructor(session: Session, from: Point, to: Point) {
    this.session = session;
    this.from = from;
    this.to = to;
  }

  moved(to: Point): Gesture {
    return new Framing(this.session, this.from, to);
  }

  now(): Session {
    return this.session.framed(new Box(this.from, this.to));
  }

  paint(ctx: CanvasRenderingContext2D): void {
    this.now().paint(ctx, new Sketch([]));
  }

  ended(): Promise<Session> {
    return Promise.resolve(this.now());
  }
}

/** Dragging the whole selection. */
export class Moving implements Gesture {
  private readonly session: Session;
  private readonly from: Point;
  private readonly to: Point;

  constructor(session: Session, from: Point, to: Point) {
    this.session = session;
    this.from = from;
    this.to = to;
  }

  moved(to: Point): Gesture {
    return new Moving(this.session, this.from, to);
  }

  now(): Session {
    return this.session.slid(this.to.minus(this.from));
  }

  paint(ctx: CanvasRenderingContext2D): void {
    this.now().paint(ctx, new Sketch([]));
  }

  ended(): Promise<Session> {
    return Promise.resolve(this.now());
  }
}

/** Resizing the selection by one of its handles. */
export class Resizing implements Gesture {
  private readonly session: Session;
  private readonly grip: Grip;
  private readonly to: Point;

  constructor(session: Session, grip: Grip, to: Point) {
    this.session = session;
    this.grip = grip;
    this.to = to;
  }

  moved(to: Point): Gesture {
    return new Resizing(this.session, this.grip, to);
  }

  now(): Session {
    return this.session.stretched(this.grip, this.to);
  }

  paint(ctx: CanvasRenderingContext2D): void {
    this.now().paint(ctx, new Sketch([]));
  }

  ended(): Promise<Session> {
    return Promise.resolve(this.now());
  }
}

/** Drawing a shape inside the selection. */
export class Drawing implements Gesture {
  private readonly session: Session;
  private readonly draft: Draft;

  constructor(session: Session, draft: Draft) {
    this.session = session;
    this.draft = draft;
  }

  moved(to: Point): Gesture {
    return new Drawing(this.session, this.draft.extended(to));
  }

  now(): Session {
    return this.session;
  }

  paint(ctx: CanvasRenderingContext2D): void {
    this.session.paint(ctx, this.draft);
  }

  ended(typing: Typing): Promise<Session> {
    return this.draft.settled(typing).then((shape) => this.session.drawn(shape));
  }
}

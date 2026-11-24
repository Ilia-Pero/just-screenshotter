import type { Box } from './box.ts';
import type { Ink } from './ink.ts';
import type { Point } from './point.ts';
import type { Draft, Mark } from './shape.ts';
import { Span } from './span.ts';

/** A straight line between two points. */
export class Segment implements Draft, Mark {
  private readonly from: Point;
  private readonly to: Point;
  private readonly ink: Ink;

  constructor(from: Point, to: Point, ink: Ink) {
    this.from = from;
    this.to = to;
    this.ink = ink;
  }

  blank(): boolean {
    return this.from.distance(this.to) < 1;
  }

  paint(ctx: CanvasRenderingContext2D): void {
    ctx.save();
    this.ink.soak(ctx);
    ctx.beginPath();
    ctx.moveTo(this.from.x, this.from.y);
    ctx.lineTo(this.to.x, this.to.y);
    ctx.stroke();
    ctx.restore();
  }

  extended(to: Point): Draft {
    return new Segment(this.from, to, this.ink);
  }

  settled(): Promise<Mark> {
    return Promise.resolve(this);
  }

  touched(at: Point, reach: number): boolean {
    return new Span(this.from, this.to).distance(at) <= reach + this.ink.times(0.5);
  }

  bounds(): Box {
    return new Span(this.from, this.to).bounds();
  }

  /** The beginning and the end. */
  handles(): ReadonlyArray<Point> {
    return [this.from, this.to];
  }

  pulled(handle: number, to: Point): Mark {
    return handle === 0 ? new Segment(to, this.to, this.ink) : new Segment(this.from, to, this.ink);
  }

  shifted(by: Point): Mark {
    return new Segment(this.from.plus(by), this.to.plus(by), this.ink);
  }

  tint(): Ink {
    return this.ink;
  }

  tinted(ink: Ink): Mark {
    return new Segment(this.from, this.to, ink);
  }

  worded(): boolean {
    return false;
  }

  retyped(): Promise<Mark> {
    return Promise.resolve(this);
  }
}

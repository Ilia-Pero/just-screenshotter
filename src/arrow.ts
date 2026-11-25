import type { Box } from './box.ts';
import type { Ink } from './ink.ts';
import { Point } from './point.ts';
import type { Draft, Mark } from './shape.ts';
import { Span } from './span.ts';

/** An arrow: a line with a triangular head at its end. */
export class Arrow implements Draft, Mark {
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

  /** Corners of the head: the tip and both ends of the base. */
  head(): readonly [Point, Point, Point] {
    const length = this.from.distance(this.to);
    const size = Math.min(this.ink.times(5), length);
    const along = this.to.minus(this.from).times(length === 0 ? 0 : size / length);
    const base = this.to.minus(along);
    const across = new Point(-along.y, along.x).times(0.45);
    return [this.to, base.plus(across), base.minus(across)];
  }

  paint(ctx: CanvasRenderingContext2D): void {
    const head = this.head();
    const tip = head[0];
    const left = head[1];
    const right = head[2];
    const neck = left.plus(right).times(0.5);
    ctx.save();
    this.ink.soak(ctx);
    ctx.beginPath();
    ctx.moveTo(this.from.x, this.from.y);
    ctx.lineTo(neck.x, neck.y);
    ctx.stroke();
    ctx.beginPath();
    ctx.moveTo(tip.x, tip.y);
    ctx.lineTo(left.x, left.y);
    ctx.lineTo(right.x, right.y);
    ctx.closePath();
    ctx.fill();
    ctx.restore();
  }

  extended(to: Point): Draft {
    return new Arrow(this.from, to, this.ink);
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
    return handle === 0 ? new Arrow(to, this.to, this.ink) : new Arrow(this.from, to, this.ink);
  }

  shifted(by: Point): Mark {
    return new Arrow(this.from.plus(by), this.to.plus(by), this.ink);
  }

  tint(): Ink {
    return this.ink;
  }

  tinted(ink: Ink): Mark {
    return new Arrow(this.from, this.to, ink);
  }

  worded(): boolean {
    return false;
  }

  retyped(): Promise<Mark> {
    return Promise.resolve(this);
  }
}

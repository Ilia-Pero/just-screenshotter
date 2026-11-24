import type { Ink } from './ink.ts';
import { Point } from './point.ts';
import type { Draft, Shape } from './shape.ts';

/** An arrow: a line with a triangular head at its end. */
export class Arrow implements Draft {
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

  settled(): Promise<Shape> {
    return Promise.resolve(this);
  }
}

import type { Ink } from './ink.ts';
import type { Point } from './point.ts';
import type { Draft, Shape } from './shape.ts';

/** A straight line between two points. */
export class Segment implements Draft {
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

  settled(): Promise<Shape> {
    return Promise.resolve(this);
  }
}

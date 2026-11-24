import { Box } from './box.ts';
import type { Ink } from './ink.ts';
import type { Point } from './point.ts';
import type { Draft, Shape } from './shape.ts';

/** A rectangular frame around something worth pointing at. */
export class Frame implements Draft {
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
    ctx.lineJoin = 'miter';
    ctx.beginPath();
    new Box(this.from, this.to).trace(ctx);
    ctx.stroke();
    ctx.restore();
  }

  extended(to: Point): Draft {
    return new Frame(this.from, to, this.ink);
  }

  settled(): Promise<Shape> {
    return Promise.resolve(this);
  }
}

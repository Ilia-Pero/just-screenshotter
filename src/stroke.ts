import type { Ink } from './ink.ts';
import type { Point } from './point.ts';
import type { Draft, Shape } from './shape.ts';

/** A freehand line: pencil or marker. */
export class Stroke implements Draft {
  private readonly points: ReadonlyArray<Point>;
  private readonly ink: Ink;

  constructor(points: ReadonlyArray<Point>, ink: Ink) {
    this.points = points;
    this.ink = ink;
  }

  blank(): boolean {
    return this.points.length === 0;
  }

  paint(ctx: CanvasRenderingContext2D): void {
    ctx.save();
    this.ink.soak(ctx);
    ctx.beginPath();
    this.points.forEach((point, index) => {
      if (index === 0) {
        ctx.moveTo(point.x, point.y);
      }
      ctx.lineTo(point.x, point.y);
    });
    ctx.stroke();
    ctx.restore();
  }

  extended(to: Point): Draft {
    return new Stroke([...this.points, to], this.ink);
  }

  settled(): Promise<Shape> {
    return Promise.resolve(this);
  }
}

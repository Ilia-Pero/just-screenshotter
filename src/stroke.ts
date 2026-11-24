import { Box } from './box.ts';
import type { Ink } from './ink.ts';
import { Point } from './point.ts';
import type { Draft, Mark } from './shape.ts';
import { Span } from './span.ts';

/** A freehand line: pencil or marker. */
export class Stroke implements Draft, Mark {
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

  settled(): Promise<Mark> {
    return Promise.resolve(this);
  }

  touched(at: Point, reach: number): boolean {
    return this.points.some(
      (point, index) =>
        new Span(this.points[index - 1] || point, point).distance(at) <=
        reach + this.ink.times(0.5),
    );
  }

  bounds(): Box {
    const across = this.points.map((point) => point.x);
    const down = this.points.map((point) => point.y);
    return new Box(
      new Point(Math.min(...across), Math.min(...down)),
      new Point(Math.max(...across), Math.max(...down)),
    );
  }

  /** A freehand line keeps its form, so there is nothing to pull it by. */
  handles(): ReadonlyArray<Point> {
    return [];
  }

  pulled(): Mark {
    return this;
  }

  shifted(by: Point): Mark {
    return new Stroke(
      this.points.map((point) => point.plus(by)),
      this.ink,
    );
  }

  tint(): Ink {
    return this.ink;
  }

  tinted(ink: Ink): Mark {
    return new Stroke(this.points, ink);
  }

  worded(): boolean {
    return false;
  }

  retyped(): Promise<Mark> {
    return Promise.resolve(this);
  }
}

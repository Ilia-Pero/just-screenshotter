import { Box } from './box.ts';
import { GRIPS } from './grip.ts';
import type { Ink } from './ink.ts';
import { Point } from './point.ts';
import type { Draft, Mark } from './shape.ts';
import { Span } from './span.ts';

/** A rectangular frame around something worth pointing at. */
export class Frame implements Draft, Mark {
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
    this.bounds().trace(ctx);
    ctx.stroke();
    ctx.restore();
  }

  extended(to: Point): Draft {
    return new Frame(this.from, to, this.ink);
  }

  settled(): Promise<Mark> {
    return Promise.resolve(this);
  }

  /** Only the line itself counts: what is inside the frame belongs to the screenshot. */
  touched(at: Point, reach: number): boolean {
    const right = new Point(this.to.x, this.from.y);
    const left = new Point(this.from.x, this.to.y);
    return [
      new Span(this.from, right),
      new Span(right, this.to),
      new Span(this.to, left),
      new Span(left, this.from),
    ].some((side) => side.distance(at) <= reach + this.ink.times(0.5));
  }

  bounds(): Box {
    return new Box(this.from, this.to);
  }

  handles(): ReadonlyArray<Point> {
    return GRIPS.map((grip) => grip.spot(this.bounds()));
  }

  pulled(handle: number, to: Point): Mark {
    return GRIPS.filter((_, index) => index === handle)
      .map((grip) => grip.pulled(this.bounds(), to))
      .reduce<Mark>(
        (_, box) =>
          new Frame(
            new Point(box.left(), box.top()),
            new Point(box.right(), box.bottom()),
            this.ink,
          ),
        this,
      );
  }

  shifted(by: Point): Mark {
    return new Frame(this.from.plus(by), this.to.plus(by), this.ink);
  }

  tint(): Ink {
    return this.ink;
  }

  tinted(ink: Ink): Mark {
    return new Frame(this.from, this.to, ink);
  }

  worded(): boolean {
    return false;
  }

  retyped(): Promise<Mark> {
    return Promise.resolve(this);
  }
}

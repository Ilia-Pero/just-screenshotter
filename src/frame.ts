import { Box } from './box.ts';
import type { Form } from './form.ts';
import { GRIPS } from './grip.ts';
import type { Ink } from './ink.ts';
import { Point } from './point.ts';
import type { Draft, Mark } from './shape.ts';

/** A frame around something worth pointing at. Its form is a rectangle or an oval. */
export class Frame implements Draft, Mark {
  private readonly from: Point;
  private readonly to: Point;
  private readonly ink: Ink;
  private readonly form: Form;

  constructor(from: Point, to: Point, ink: Ink, form: Form) {
    this.from = from;
    this.to = to;
    this.ink = ink;
    this.form = form;
  }

  blank(): boolean {
    return this.from.distance(this.to) < 1;
  }

  paint(ctx: CanvasRenderingContext2D): void {
    ctx.save();
    this.ink.soak(ctx);
    ctx.lineJoin = 'miter';
    ctx.beginPath();
    this.form.trace(ctx, this.bounds());
    ctx.stroke();
    ctx.restore();
  }

  extended(to: Point, even: boolean): Draft {
    return new Frame(this.from, even ? to.squared(this.from) : to, this.ink, this.form);
  }

  settled(): Promise<Mark> {
    return Promise.resolve(this);
  }

  /** Only the line itself counts: what is inside the frame belongs to the screenshot. */
  touched(at: Point, reach: number): boolean {
    return this.form.distance(this.bounds(), at) <= reach + this.ink.times(0.5);
  }

  bounds(): Box {
    return new Box(this.from, this.to);
  }

  handles(): ReadonlyArray<Point> {
    return GRIPS.map((grip) => grip.spot(this.bounds()));
  }

  pulled(handle: number, to: Point, even: boolean): Mark {
    return GRIPS.filter((_, index) => index === handle)
      .map((grip) =>
        even ? grip.squared(this.bounds(), to) : grip.pulled(this.bounds(), to),
      )
      .reduce<Mark>(
        (_, box) =>
          new Frame(
            new Point(box.left(), box.top()),
            new Point(box.right(), box.bottom()),
            this.ink,
            this.form,
          ),
        this,
      );
  }

  shifted(by: Point): Mark {
    return new Frame(this.from.plus(by), this.to.plus(by), this.ink, this.form);
  }

  tint(): Ink {
    return this.ink;
  }

  tinted(ink: Ink): Mark {
    return new Frame(this.from, this.to, ink, this.form);
  }

  worded(): boolean {
    return false;
  }

  retyped(): Promise<Mark> {
    return Promise.resolve(this);
  }
}

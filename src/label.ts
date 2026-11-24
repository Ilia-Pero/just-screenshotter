import { Box } from './box.ts';
import type { Ink } from './ink.ts';
import { Point } from './point.ts';
import type { Ruler } from './ruler.ts';
import { font } from './ruler.ts';
import type { Mark, Typing } from './shape.ts';

/**
 * A line of text. The point is the top left corner of the line, the thickness of the ink is
 * the size of the letters.
 */
export class Label implements Mark {
  private readonly at: Point;
  private readonly text: string;
  private readonly ink: Ink;
  private readonly ruler: Ruler;

  constructor(at: Point, text: string, ink: Ink, ruler: Ruler) {
    this.at = at;
    this.text = text;
    this.ink = ink;
    this.ruler = ruler;
  }

  blank(): boolean {
    return this.text.trim() === '';
  }

  paint(ctx: CanvasRenderingContext2D): void {
    ctx.save();
    this.ink.soak(ctx);
    ctx.font = font(this.ink.times(1));
    ctx.textBaseline = 'middle';
    ctx.fillText(this.text, this.at.x, this.at.y + this.ink.times(0.6));
    ctx.restore();
  }

  touched(at: Point, reach: number): boolean {
    return this.bounds().nearest(at).distance(at) <= reach;
  }

  bounds(): Box {
    return new Box(
      this.at,
      this.at.plus(new Point(this.ruler.width(this.text, this.ink.times(1)), this.ink.times(1.2))),
    );
  }

  /** A text has no ends to pull: the size of its letters is set like the thickness of a line. */
  handles(): ReadonlyArray<Point> {
    return [];
  }

  pulled(): Mark {
    return this;
  }

  shifted(by: Point): Mark {
    return new Label(this.at.plus(by), this.text, this.ink, this.ruler);
  }

  tint(): Ink {
    return this.ink;
  }

  tinted(ink: Ink): Mark {
    return new Label(this.at, this.text, ink, this.ruler);
  }

  worded(): boolean {
    return true;
  }

  retyped(typing: Typing): Promise<Mark> {
    return typing
      .words(this.at, this.ink, this.text)
      .then((text) =>
        text === this.text ? this : new Label(this.at, text, this.ink, this.ruler),
      );
  }
}

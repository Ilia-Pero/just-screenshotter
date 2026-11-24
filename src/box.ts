import { Point } from './point.ts';

/** A rectangle. Its two corners may come in any order. */
export class Box {
  private readonly first: Point;
  private readonly second: Point;

  constructor(first: Point, second: Point) {
    this.first = first;
    this.second = second;
  }

  left(): number {
    return Math.min(this.first.x, this.second.x);
  }

  top(): number {
    return Math.min(this.first.y, this.second.y);
  }

  right(): number {
    return Math.max(this.first.x, this.second.x);
  }

  bottom(): number {
    return Math.max(this.first.y, this.second.y);
  }

  width(): number {
    return this.right() - this.left();
  }

  height(): number {
    return this.bottom() - this.top();
  }

  /** True if there is not a single pixel inside. */
  blank(): boolean {
    return this.width() < 1 || this.height() < 1;
  }

  holds(point: Point): boolean {
    return (
      point.x >= this.left() &&
      point.x <= this.right() &&
      point.y >= this.top() &&
      point.y <= this.bottom()
    );
  }

  /** The point inside the rectangle that is closest to the given one. */
  nearest(point: Point): Point {
    return new Point(
      Math.min(Math.max(point.x, this.left()), this.right()),
      Math.min(Math.max(point.y, this.top()), this.bottom()),
    );
  }

  shifted(by: Point): Box {
    return new Box(this.first.plus(by), this.second.plus(by));
  }

  /** The same rectangle with every coordinate multiplied by the factor. */
  scaled(factor: number): Box {
    return new Box(this.first.times(factor), this.second.times(factor));
  }

  /** The same rectangle, moved just enough to stay within the frame. */
  inside(frame: Box): Box {
    return this.shifted(
      new Point(
        Math.max(frame.left() - this.left(), 0) + Math.min(frame.right() - this.right(), 0),
        Math.max(frame.top() - this.top(), 0) + Math.min(frame.bottom() - this.bottom(), 0),
      ),
    );
  }

  /** The part of the rectangle that fits into the frame. */
  clipped(frame: Box): Box {
    return new Box(frame.nearest(this.first), frame.nearest(this.second));
  }

  /** Adds the outline to the current path of the canvas. */
  trace(ctx: CanvasRenderingContext2D): void {
    ctx.rect(this.left(), this.top(), this.width(), this.height());
  }
}

/** A point on the screenshot, in screenshot pixels. */
export class Point {
  readonly x: number;
  readonly y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }

  plus(other: Point): Point {
    return new Point(this.x + other.x, this.y + other.y);
  }

  minus(other: Point): Point {
    return new Point(this.x - other.x, this.y - other.y);
  }

  times(factor: number): Point {
    return new Point(this.x * factor, this.y * factor);
  }

  /** The corner of a square whose opposite corner is the other point, as large as this allows. */
  squared(other: Point): Point {
    const by = this.minus(other);
    const side = Math.max(Math.abs(by.x), Math.abs(by.y));
    return other.plus(new Point(by.x < 0 ? -side : side, by.y < 0 ? -side : side));
  }

  distance(other: Point): number {
    return Math.hypot(this.x - other.x, this.y - other.y);
  }
}

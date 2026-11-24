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

  distance(other: Point): number {
    return Math.hypot(this.x - other.x, this.y - other.y);
  }
}

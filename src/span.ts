import { Box } from './box.ts';
import type { Point } from './point.ts';

/** A straight stretch between two points. */
export class Span {
  private readonly from: Point;
  private readonly to: Point;

  constructor(from: Point, to: Point) {
    this.from = from;
    this.to = to;
  }

  /** How far the point is from the closest point of the stretch. */
  distance(at: Point): number {
    const along = this.to.minus(this.from);
    const whole = along.x * along.x + along.y * along.y;
    const toward = at.minus(this.from);
    const share = whole === 0 ? 0 : (toward.x * along.x + toward.y * along.y) / whole;
    return this.from.plus(along.times(Math.min(Math.max(share, 0), 1))).distance(at);
  }

  bounds(): Box {
    return new Box(this.from, this.to);
  }
}

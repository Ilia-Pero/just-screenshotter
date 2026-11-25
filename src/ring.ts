import type { Point } from './point.ts';
import { Span } from './span.ts';

/** A closed line through points. */
export class Ring {
  private readonly points: ReadonlyArray<Point>;

  constructor(points: ReadonlyArray<Point>) {
    this.points = points;
  }

  /** How far the point is from the line. */
  distance(at: Point): number {
    return Math.min(
      ...this.points.map((point, index) =>
        new Span(this.points[index - 1] ?? this.points.slice(-1)[0] ?? point, point).distance(at),
      ),
    );
  }
}

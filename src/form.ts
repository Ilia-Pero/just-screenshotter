import type { Box } from './box.ts';
import { Grip } from './grip.ts';
import { Point } from './point.ts';
import { Ring } from './ring.ts';

/** How many straight sides stand for an oval when the distance to it is measured. */
const SIDES = 96;

/** The line of a frame inside a rectangle. */
export interface Form {
  /** Adds the line to the current path of the canvas. */
  trace(ctx: CanvasRenderingContext2D, box: Box): void;
  /** How far the point is from the line. */
  distance(box: Box, at: Point): number;
}

/** The border of the rectangle. */
export class Border implements Form {
  trace(ctx: CanvasRenderingContext2D, box: Box): void {
    box.trace(ctx);
  }

  distance(box: Box, at: Point): number {
    return new Ring(
      [new Grip(-1, -1), new Grip(1, -1), new Grip(1, 1), new Grip(-1, 1)].map((corner) =>
        corner.spot(box),
      ),
    ).distance(at);
  }
}

/** The oval that touches the sides of the rectangle. */
export class Oval implements Form {
  trace(ctx: CanvasRenderingContext2D, box: Box): void {
    const middle = new Grip(0, 0).spot(box);
    ctx.ellipse(middle.x, middle.y, box.width() / 2, box.height() / 2, 0, 0, 2 * Math.PI);
  }

  distance(box: Box, at: Point): number {
    const middle = new Grip(0, 0).spot(box);
    return new Ring(
      Array.from({ length: SIDES }, (_, index) => (2 * Math.PI * index) / SIDES).map((angle) =>
        middle.plus(
          new Point((box.width() / 2) * Math.cos(angle), (box.height() / 2) * Math.sin(angle)),
        ),
      ),
    ).distance(at);
  }
}

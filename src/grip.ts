import { Box } from './box.ts';
import { Point } from './point.ts';

/**
 * A handle on the border of a rectangle, used to resize it.
 * across: -1 is the left side, 0 the middle, 1 the right side.
 * down: -1 is the top side, 0 the middle, 1 the bottom side.
 */
export class Grip {
  private readonly across: number;
  private readonly down: number;

  constructor(across: number, down: number) {
    this.across = across;
    this.down = down;
  }

  /** Where the handle sits on the given rectangle. */
  spot(box: Box): Point {
    return new Point(
      (box.left() + box.right() + this.across * box.width()) / 2,
      (box.top() + box.bottom() + this.down * box.height()) / 2,
    );
  }

  near(box: Box, at: Point, reach: number): boolean {
    return this.spot(box).distance(at) <= reach;
  }

  /** The rectangle after the handle has been dragged to the point. */
  pulled(box: Box, to: Point): Box {
    return new Box(
      new Point(this.across < 0 ? to.x : box.left(), this.down < 0 ? to.y : box.top()),
      new Point(this.across > 0 ? to.x : box.right(), this.down > 0 ? to.y : box.bottom()),
    );
  }

  /** The same, but a handle at a corner makes the rectangle a square. */
  squared(box: Box, to: Point): Box {
    return this.across === 0 || this.down === 0
      ? this.pulled(box, to)
      : this.pulled(box, to.squared(new Grip(-this.across, -this.down).spot(box)));
  }

  cursor(): string {
    if (this.across === 0) {
      return 'ns-resize';
    }
    if (this.down === 0) {
      return 'ew-resize';
    }
    return this.across === this.down ? 'nwse-resize' : 'nesw-resize';
  }
}

/** All handles of a rectangle. */
export const GRIPS: ReadonlyArray<Grip> = [
  new Grip(-1, -1),
  new Grip(0, -1),
  new Grip(1, -1),
  new Grip(-1, 0),
  new Grip(1, 0),
  new Grip(-1, 1),
  new Grip(0, 1),
  new Grip(1, 1),
];

import type { Box } from './box.ts';
import { Point } from './point.ts';

/** The size of a panel, in pixels of the window. */
export interface Size {
  readonly width: number;
  readonly height: number;
}

/** The distance between the selection and the panels. */
const GAP = 8;

/**
 * Where the panels stand around the selection. They stay on the screen even when the selection
 * touches its edges. Everything is measured in pixels of the window.
 */
export class Layout {
  private readonly window: Size;
  private readonly area: Box;
  private readonly tools: Size;
  private readonly actions: Size;
  private readonly badge: Size;

  constructor(window: Size, area: Box, tools: Size, actions: Size, badge: Size) {
    this.window = window;
    this.area = area;
    this.tools = tools;
    this.actions = actions;
    this.badge = badge;
  }

  /** The column of drawing tools: to the right of the selection, or inside it. */
  toolsSpot(): Point {
    const floor = Math.min(this.area.bottom(), this.window.height - GAP);
    return this.kept(new Point(this.column(), floor - this.tools.height));
  }

  /** The row of actions: below the selection, above it, or inside it. */
  actionsSpot(): Point {
    const edge = this.beside() ? this.area.right() : this.column() - GAP;
    return this.kept(new Point(edge - this.actions.width, this.row()));
  }

  /** The size of the selection: above it, or inside it. */
  badgeSpot(): Point {
    return this.kept(
      new Point(
        this.area.left(),
        this.above() ? this.area.top() - GAP - this.badge.height : this.area.top() + GAP,
      ),
    );
  }

  private column(): number {
    return this.beside() ? this.area.right() + GAP : this.area.right() - GAP - this.tools.width;
  }

  private row(): number {
    if (this.area.bottom() + GAP + this.actions.height <= this.window.height) {
      return this.area.bottom() + GAP;
    }
    if (this.above()) {
      return this.area.top() - GAP - this.actions.height;
    }
    return this.area.bottom() - GAP - this.actions.height;
  }

  private beside(): boolean {
    return this.area.right() + GAP + this.tools.width <= this.window.width;
  }

  private above(): boolean {
    return this.area.top() - GAP - this.actions.height >= 0;
  }

  private kept(spot: Point): Point {
    return new Point(Math.max(GAP, spot.x), Math.max(GAP, spot.y));
  }
}

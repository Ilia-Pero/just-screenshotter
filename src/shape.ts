import type { Ink } from './ink.ts';
import type { Point } from './point.ts';

/** Something drawn over the screenshot. */
export interface Shape {
  /** True if there is nothing to draw. */
  blank(): boolean;
  paint(ctx: CanvasRenderingContext2D): void;
}

/** A shape that is still being dragged with the mouse. */
export interface Draft extends Shape {
  /** The same shape, stretched to the point. */
  extended(to: Point): Draft;
  /** The finished shape, once the mouse button is up. */
  settled(typing: Typing): Promise<Shape>;
}

/** The place where a person types a label. */
export interface Typing {
  words(at: Point, ink: Ink, size: number): Promise<string>;
}

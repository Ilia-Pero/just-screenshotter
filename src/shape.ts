import type { Box } from './box.ts';
import type { Ink } from './ink.ts';
import type { Point } from './point.ts';

/** Something drawn over the screenshot. */
export interface Shape {
  /** True if there is nothing to draw. */
  blank(): boolean;
  paint(ctx: CanvasRenderingContext2D): void;
}

/** A finished shape. It can be picked later and changed. */
export interface Mark extends Shape {
  /** True if the point is on the mark or within reach of it. */
  touched(at: Point, reach: number): boolean;
  /** The smallest rectangle around the mark. */
  bounds(): Box;
  /** Where the handles are. A mark without handles can be dragged only as a whole. */
  handles(): ReadonlyArray<Point>;
  /** The mark after the handle with the number has been dragged to the point. */
  pulled(handle: number, to: Point): Mark;
  shifted(by: Point): Mark;
  /** What the mark is drawn with. */
  tint(): Ink;
  tinted(ink: Ink): Mark;
  /** True if the mark is a text. */
  worded(): boolean;
  /** The mark with its text typed anew. */
  retyped(typing: Typing): Promise<Mark>;
}

/** A shape that is still being dragged with the mouse. */
export interface Draft extends Shape {
  /** The same shape, stretched to the point. */
  extended(to: Point): Draft;
  /** The finished shape, once the mouse button is up. */
  settled(typing: Typing): Promise<Mark>;
}

/** The place where a person types a label. */
export interface Typing {
  /** The text typed at the point. At first the place holds the given text. */
  words(at: Point, ink: Ink, text: string): Promise<string>;
}

/** Nothing at all. */
export class Nothing implements Shape {
  blank(): boolean {
    return true;
  }

  paint(): void {
    // There is nothing to draw.
  }
}

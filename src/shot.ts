import type { Box } from './box.ts';

/** The screen as it was when the tool was called. */
export interface Shot {
  /** From zero to the width and height of the screenshot, in its pixels. */
  bounds(): Box;
  paint(ctx: CanvasRenderingContext2D): void;
}

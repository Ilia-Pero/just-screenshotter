/** Tells how wide texts are when they are drawn. */
export interface Ruler {
  /** The width of the text with letters of the given size. */
  width(text: string, size: number): number;
}

/** The font of labels with letters of the given size. */
export function font(size: number): string {
  return `600 ${size}px sans-serif`;
}

/** The ruler of a canvas. */
export class Tape implements Ruler {
  private readonly ctx: CanvasRenderingContext2D;

  constructor(ctx: CanvasRenderingContext2D) {
    this.ctx = ctx;
  }

  width(text: string, size: number): number {
    this.ctx.save();
    this.ctx.font = font(size);
    const width = this.ctx.measureText(text).width;
    this.ctx.restore();
    return width;
  }
}

import type { Ink } from './ink.ts';
import type { Point } from './point.ts';
import type { Shape } from './shape.ts';

/** A line of text. The point is the top left corner of the line. */
export class Label implements Shape {
  private readonly at: Point;
  private readonly text: string;
  private readonly ink: Ink;
  private readonly size: number;

  constructor(at: Point, text: string, ink: Ink, size: number) {
    this.at = at;
    this.text = text;
    this.ink = ink;
    this.size = size;
  }

  blank(): boolean {
    return this.text.trim() === '';
  }

  paint(ctx: CanvasRenderingContext2D): void {
    ctx.save();
    this.ink.soak(ctx);
    ctx.font = `600 ${this.size}px sans-serif`;
    ctx.textBaseline = 'middle';
    ctx.fillText(this.text, this.at.x, this.at.y + this.size * 0.6);
    ctx.restore();
  }
}

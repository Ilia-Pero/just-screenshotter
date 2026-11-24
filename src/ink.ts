/** What to draw with: colour, line width in screenshot pixels, and opacity. */
export class Ink {
  private readonly color: string;
  private readonly width: number;
  private readonly alpha: number;

  constructor(color: string, width: number, alpha: number) {
    this.color = color;
    this.width = width;
    this.alpha = alpha;
  }

  /** Makes the canvas draw with this ink. */
  soak(ctx: CanvasRenderingContext2D): void {
    ctx.strokeStyle = this.color;
    ctx.fillStyle = this.color;
    ctx.lineWidth = this.width;
    ctx.globalAlpha = this.alpha;
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';
  }

  /** The line width multiplied by the factor. */
  times(factor: number): number {
    return this.width * factor;
  }

  /** The colour in CSS notation. */
  css(): string {
    return this.color;
  }
}

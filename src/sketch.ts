import type { Shape } from './shape.ts';

/** Everything drawn over the screenshot, in the order of drawing. */
export class Sketch implements Shape {
  private readonly shapes: ReadonlyArray<Shape>;

  constructor(shapes: ReadonlyArray<Shape>) {
    this.shapes = shapes;
  }

  blank(): boolean {
    return this.shapes.length === 0;
  }

  paint(ctx: CanvasRenderingContext2D): void {
    this.shapes.forEach((shape) => shape.paint(ctx));
  }

  plus(shape: Shape): Sketch {
    return new Sketch([...this.shapes, shape]);
  }

  /** The sketch without its last shape. */
  undone(): Sketch {
    return new Sketch(this.shapes.slice(0, -1));
  }
}

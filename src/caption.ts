import type { Ink } from './ink.ts';
import { Label } from './label.ts';
import type { Point } from './point.ts';
import type { Draft, Shape, Typing } from './shape.ts';

/** The place for a label: there is no text yet, it is typed after the click. */
export class Caption implements Draft {
  private readonly at: Point;
  private readonly ink: Ink;
  private readonly size: number;

  constructor(at: Point, ink: Ink, size: number) {
    this.at = at;
    this.ink = ink;
    this.size = size;
  }

  blank(): boolean {
    return true;
  }

  paint(): void {
    // Nothing to draw until the text is typed: Typing shows the input field.
  }

  extended(): Draft {
    return this;
  }

  settled(typing: Typing): Promise<Shape> {
    return typing
      .words(this.at, this.ink, this.size)
      .then((text) => new Label(this.at, text, this.ink, this.size));
  }
}

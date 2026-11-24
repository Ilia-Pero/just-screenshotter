import type { Ink } from './ink.ts';
import { Label } from './label.ts';
import type { Point } from './point.ts';
import type { Ruler } from './ruler.ts';
import type { Draft, Mark, Typing } from './shape.ts';

/** The place for a label: there is no text yet, it is typed after the click. */
export class Caption implements Draft {
  private readonly at: Point;
  private readonly ink: Ink;
  private readonly ruler: Ruler;

  constructor(at: Point, ink: Ink, ruler: Ruler) {
    this.at = at;
    this.ink = ink;
    this.ruler = ruler;
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

  settled(typing: Typing): Promise<Mark> {
    return new Label(this.at, '', this.ink, this.ruler).retyped(typing);
  }
}

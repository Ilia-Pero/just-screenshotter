import type { Gesture } from './gesture.ts';
import { Drawing } from './gesture.ts';
import { Ink } from './ink.ts';
import type { Point } from './point.ts';
import type { Session } from './session.ts';
import type { Draft } from './shape.ts';

/** The tool chosen on the panel: what the mouse does inside the selection. */
export interface Tool {
  /** The cursor. over tells whether there is a mark under it. */
  cursor(over: boolean): string;
  gesture(session: Session, at: Point, color: string): Gesture;
  /** The thickness of what it draws. Zero if it does not draw. */
  thickness(): number;
  /** The same tool that draws with another thickness. */
  thick(width: number): Tool;
}

/** No tool is chosen: the mouse picks and moves the marks, and moves the selection. */
export class Hand implements Tool {
  cursor(over: boolean): string {
    return over ? 'pointer' : 'move';
  }

  gesture(session: Session, at: Point): Gesture {
    return session.grab(at);
  }

  thickness(): number {
    return 0;
  }

  thick(): Tool {
    return this;
  }
}

/** A drawing tool: thickness, opacity and the shape it starts. */
export class Brush implements Tool {
  private readonly width: number;
  private readonly alpha: number;
  private readonly sprout: (at: Point, ink: Ink) => Draft;

  constructor(width: number, alpha: number, sprout: (at: Point, ink: Ink) => Draft) {
    this.width = width;
    this.alpha = alpha;
    this.sprout = sprout;
  }

  cursor(over: boolean): string {
    return over ? 'pointer' : 'crosshair';
  }

  gesture(session: Session, at: Point, color: string): Gesture {
    return session.touch(
      at,
      new Drawing(session.released(), this.sprout(at, new Ink(color, this.width, this.alpha))),
    );
  }

  thickness(): number {
    return this.width;
  }

  thick(width: number): Tool {
    return new Brush(width, this.alpha, this.sprout);
  }
}

/** A tool for texts: a press on a text that is already there changes it. */
export class Quill implements Tool {
  private readonly brush: Tool;

  constructor(brush: Tool) {
    this.brush = brush;
  }

  cursor(): string {
    return 'text';
  }

  gesture(session: Session, at: Point, color: string): Gesture {
    return session.reword(at, this.brush.gesture(session, at, color));
  }

  thickness(): number {
    return this.brush.thickness();
  }

  thick(width: number): Tool {
    return new Quill(this.brush.thick(width));
  }
}

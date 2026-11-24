import type { Gesture } from './gesture.ts';
import { Drawing, Moving } from './gesture.ts';
import { Ink } from './ink.ts';
import type { Point } from './point.ts';
import type { Session } from './session.ts';
import type { Draft } from './shape.ts';

/** The tool chosen on the panel: what the mouse does inside the selection. */
export interface Tool {
  cursor(): string;
  gesture(session: Session, at: Point, color: string): Gesture;
}

/** No tool is chosen: the mouse moves the selection. */
export class Hand implements Tool {
  cursor(): string {
    return 'move';
  }

  gesture(session: Session, at: Point): Gesture {
    return new Moving(session, at, at);
  }
}

/** A drawing tool: line width, opacity and the shape it starts. */
export class Brush implements Tool {
  private readonly width: number;
  private readonly alpha: number;
  private readonly sign: string;
  private readonly sprout: (at: Point, ink: Ink) => Draft;

  constructor(width: number, alpha: number, sign: string, sprout: (at: Point, ink: Ink) => Draft) {
    this.width = width;
    this.alpha = alpha;
    this.sign = sign;
    this.sprout = sprout;
  }

  cursor(): string {
    return this.sign;
  }

  gesture(session: Session, at: Point, color: string): Gesture {
    return new Drawing(session, this.sprout(at, new Ink(color, this.width, this.alpha)));
  }
}

import { Box } from '../src/box.ts';
import { Ink } from '../src/ink.ts';
import { Point } from '../src/point.ts';
import { Session } from '../src/session.ts';
import type { Ruler } from '../src/ruler.ts';
import type { Typing } from '../src/shape.ts';
import type { Shot } from '../src/shot.ts';
import { Sketch } from '../src/sketch.ts';
import type { Tool } from '../src/tool.ts';

/** A blank sheet instead of the screen. */
export class Sheet implements Shot {
  private readonly width: number;
  private readonly height: number;

  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
  }

  bounds(): Box {
    return new Box(new Point(0, 0), new Point(this.width, this.height));
  }

  paint(): void {
    // There is nothing to draw on a blank sheet.
  }
}

/** A ruler for which every letter is as wide as half of its size. */
export class Even implements Ruler {
  width(text: string, size: number): number {
    return (text.length * size) / 2;
  }
}

/** A text known in advance instead of one typed by a person. */
export class Script implements Typing {
  private readonly text: string;

  constructor(text: string) {
    this.text = text;
  }

  words(): Promise<string> {
    return Promise.resolve(this.text);
  }
}

export function point(x: number, y: number): Point {
  return new Point(x, y);
}

export function box(left: number, top: number, right: number, bottom: number): Box {
  return new Box(new Point(left, top), new Point(right, bottom));
}

export function ink(width: number): Ink {
  return new Ink('#000000', width, 1);
}

/** A session over a sheet of 100 by 80, with nothing selected and the given tool. */
export function fresh(tool: Tool): Session {
  return new Session(new Sheet(100, 80), box(0, 0, 0, 0), new Sketch([], []), tool, '#000000', 1);
}

/** The session after the mouse is dragged from one point to another. */
export function dragged(session: Session, from: Point, to: Point, text: string): Promise<Session> {
  return session.press(from).moved(to).ended(new Script(text));
}

/** A session over a sheet of 100 by 80, where the area from (10, 10) to (60, 40) is selected. */
export function framed(tool: Tool): Promise<Session> {
  return dragged(fresh(tool), point(10, 10), point(60, 40), '');
}

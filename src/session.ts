import type { Box } from './box.ts';
import { Cut } from './cut.ts';
import type { Gesture } from './gesture.ts';
import { Framing, Moving, Pulling, Resizing, Rest, Retyping, Shifting } from './gesture.ts';
import type { Grip } from './grip.ts';
import { GRIPS } from './grip.ts';
import type { Point } from './point.ts';
import type { Mark, Shape, Typing } from './shape.ts';
import type { Shot } from './shot.ts';
import type { Sketch } from './sketch.ts';
import type { Tool } from './tool.ts';

/**
 * Work on one screenshot: what is selected, what is drawn and with what.
 * unit is the number of screenshot pixels in one pixel of the window.
 */
export class Session {
  private readonly shot: Shot;
  private readonly box: Box;
  private readonly sketch: Sketch;
  private readonly tool: Tool;
  private readonly color: string;
  private readonly unit: number;

  constructor(shot: Shot, box: Box, sketch: Sketch, tool: Tool, color: string, unit: number) {
    this.shot = shot;
    this.box = box;
    this.sketch = sketch;
    this.tool = tool;
    this.color = color;
    this.unit = unit;
  }

  /** The session with another selection, clipped by the edges of the screenshot. */
  framed(box: Box): Session {
    return this.with(box.clipped(this.shot.bounds()), this.sketch);
  }

  /** The session where the selection is moved but stays within the screenshot. */
  slid(by: Point): Session {
    return this.with(this.box.shifted(by).inside(this.shot.bounds()), this.sketch);
  }

  stretched(grip: Grip, to: Point): Session {
    return this.framed(grip.pulled(this.box, to));
  }

  /**
   * The session with one more mark. A blank one changes nothing. The mark is picked at once,
   * except a freehand line: they are drawn in a row, and the outline would be in the way.
   */
  drawn(mark: Mark): Session {
    if (mark.blank()) {
      return this;
    }
    const sketch = this.sketch.plus(mark);
    const changeable = mark.worded() || mark.handles().length > 0;
    return this.with(this.box, changeable ? sketch.latest() : sketch);
  }

  undone(): Session {
    return this.with(this.box, this.sketch.undone());
  }

  equipped(tool: Tool): Session {
    return new Session(
      this.shot,
      this.box,
      this.sketch.released(),
      tool,
      this.color,
      this.unit,
    );
  }

  /** The session with another colour: of the picked mark and of the ones to come. */
  dyed(color: string): Session {
    return new Session(
      this.shot,
      this.box,
      this.sketch.changed((mark) => mark.tinted(mark.tint().dyed(color)), ''),
      this.tool,
      color,
      this.unit,
    );
  }

  /** The session with another thickness: of the picked mark, or of the tool if none is picked. */
  thick(width: number): Session {
    return this.picked()
      ? this.with(
          this.box,
          this.sketch.changed((mark) => mark.tinted(mark.tint().thick(width)), 'thick'),
        )
      : new Session(
          this.shot,
          this.box,
          this.sketch,
          this.tool.thick(width),
          this.color,
          this.unit,
        );
  }

  /** The session where no mark is picked. */
  released(): Session {
    return this.with(this.box, this.sketch.released());
  }

  /** The session without the picked mark. */
  dropped(): Session {
    return this.with(this.box, this.sketch.dropped());
  }

  /** The session where the picked mark is moved. */
  shifted(by: Point): Session {
    return by.x === 0 && by.y === 0
      ? this
      : this.with(
          this.box,
          this.sketch.changed((mark) => mark.shifted(by), ''),
        );
  }

  /** The session where the picked mark is pulled by one of its handles. */
  pulled(handle: number, to: Point): Session {
    return this.with(
      this.box,
      this.sketch.changed((mark) => mark.pulled(handle, to), ''),
    );
  }

  /** The session with the text of the picked mark typed anew. An empty text removes the mark. */
  retyped(typing: Typing): Promise<Session> {
    return this.sketch.picked().reduce<Promise<Session>>(
      (before, mark) =>
        before.then((session) =>
          mark.retyped(typing).then((typed) => {
            if (typed === mark) {
              return session;
            }
            return typed.blank()
              ? session.dropped()
              : session.with(
                  session.box,
                  session.sketch.changed(() => typed, ''),
                );
          }),
        ),
      Promise.resolve(this),
    );
  }

  /** True if something is drawn on the screenshot. */
  marked(): boolean {
    return !this.sketch.blank();
  }

  /** True if there is something to undo. */
  begun(): boolean {
    return this.sketch.begun();
  }

  /** True if a mark is picked. */
  picked(): boolean {
    return this.sketch.picked().length > 0;
  }

  /** The thickness of the picked mark, or of the tool if none is picked. Zero if there is none. */
  thickness(): number {
    return this.sketch
      .picked()
      .reduce((_, mark) => mark.tint().times(1), this.tool.thickness());
  }

  /** The colour of the picked mark, or of the marks to come if none is picked. */
  hue(): string {
    return this.sketch.picked().reduce((_, mark) => mark.tint().css(), this.color);
  }

  /** The selection. */
  place(): Box {
    return this.box;
  }

  /** The movement that starts when the mouse button is pressed at the point. */
  press(at: Point): Gesture {
    if (this.box.blank()) {
      return new Framing(this, at, at);
    }
    const handle = this.handle(at);
    if (handle >= 0) {
      return new Pulling(this, handle, at, at);
    }
    for (const grip of GRIPS) {
      if (grip.near(this.box, at, this.reach())) {
        return new Resizing(this, grip, at);
      }
    }
    return this.box.holds(at)
      ? this.tool.gesture(this, at, this.color)
      : new Framing(this.released(), at, at);
  }

  /** Picking the mark at the point and dragging it; without a mark, dragging the selection. */
  grab(at: Point): Gesture {
    const next = this.picking(at);
    return next.picked() ? new Shifting(next, at, at) : new Moving(next, at, at);
  }

  /** Typing the text at the point anew. If there is no text, the other movement. */
  reword(at: Point, otherwise: Gesture): Gesture {
    const next = this.picking(at);
    return next.worded() ? new Retyping(next) : otherwise;
  }

  /** Typing the text of the picked mark anew, if it has a text. */
  rewrite(): Gesture {
    return this.worded() ? new Retyping(this) : new Rest(this);
  }

  /** The shape of the cursor over the point. */
  cursor(at: Point): string {
    if (this.box.blank()) {
      return 'crosshair';
    }
    if (this.handle(at) >= 0) {
      return 'pointer';
    }
    for (const grip of GRIPS) {
      if (grip.near(this.box, at, this.reach())) {
        return grip.cursor();
      }
    }
    return this.box.holds(at) ? this.tool.cursor(this.picking(at).picked()) : 'crosshair';
  }

  /** Draws the screenshot, the shade around the selection, the sketch and the current draft. */
  paint(ctx: CanvasRenderingContext2D, draft: Shape): void {
    this.shot.paint(ctx);
    ctx.save();
    ctx.fillStyle = 'rgba(0, 0, 0, 0.5)';
    ctx.beginPath();
    this.shot.bounds().trace(ctx);
    this.box.trace(ctx);
    ctx.fill('evenodd');
    ctx.beginPath();
    this.box.trace(ctx);
    ctx.clip();
    this.sketch.paint(ctx);
    draft.paint(ctx);
    ctx.restore();
    if (!this.box.blank()) {
      this.outline(ctx);
      this.sketch.picked().forEach((mark) => this.point(ctx, mark));
    }
  }

  cut(): Cut {
    return new Cut(this.shot, this.box, this.sketch);
  }

  private with(box: Box, sketch: Sketch): Session {
    return new Session(this.shot, box, sketch, this.tool, this.color, this.unit);
  }

  private picking(at: Point): Session {
    return this.with(this.box, this.sketch.picking(at, 5 * this.unit));
  }

  private worded(): boolean {
    return this.sketch.picked().some((mark) => mark.worded());
  }

  /** The number of the handle of the picked mark at the point. Negative if there is none. */
  private handle(at: Point): number {
    const spots = this.sketch
      .picked()
      .reduce<ReadonlyArray<Point>>((_, mark) => mark.handles(), [])
      .map((spot) => spot.distance(at));
    const closest = Math.min(...spots);
    return closest <= this.reach() ? spots.indexOf(closest) : -1;
  }

  /** How far from a handle a press still counts as a press on the handle. */
  private reach(): number {
    return 8 * this.unit;
  }

  private outline(ctx: CanvasRenderingContext2D): void {
    const half = 3 * this.unit;
    ctx.save();
    this.dashes(ctx, this.box);
    ctx.fillStyle = '#fff';
    ctx.strokeStyle = '#000';
    GRIPS.forEach((grip) => {
      const spot = grip.spot(this.box);
      ctx.fillRect(spot.x - half, spot.y - half, half * 2, half * 2);
      ctx.strokeRect(spot.x - half, spot.y - half, half * 2, half * 2);
    });
    ctx.restore();
  }

  /** Shows that the mark is picked: its handles, or a line around it if it has none. */
  private point(ctx: CanvasRenderingContext2D, mark: Mark): void {
    const spots = mark.handles();
    ctx.save();
    if (spots.length === 0) {
      this.dashes(ctx, mark.bounds().padded(4 * this.unit + mark.tint().times(0.1)));
    }
    ctx.lineWidth = 1.5 * this.unit;
    ctx.fillStyle = '#fff';
    ctx.strokeStyle = '#2f6fdf';
    spots.forEach((spot) => {
      ctx.beginPath();
      ctx.arc(spot.x, spot.y, 5 * this.unit, 0, 2 * Math.PI);
      ctx.fill();
      ctx.stroke();
    });
    ctx.restore();
  }

  private dashes(ctx: CanvasRenderingContext2D, box: Box): void {
    ctx.lineWidth = this.unit;
    ctx.strokeStyle = '#000';
    ctx.beginPath();
    box.trace(ctx);
    ctx.stroke();
    ctx.strokeStyle = '#fff';
    ctx.setLineDash([4 * this.unit, 4 * this.unit]);
    ctx.stroke();
    ctx.setLineDash([]);
  }
}

import type { Box } from './box.ts';
import { Cut } from './cut.ts';
import type { Gesture } from './gesture.ts';
import { Framing, Resizing } from './gesture.ts';
import { Grip } from './grip.ts';
import type { Point } from './point.ts';
import type { Shape } from './shape.ts';
import type { Shot } from './shot.ts';
import type { Sketch } from './sketch.ts';
import type { Tool } from './tool.ts';

const GRIPS: ReadonlyArray<Grip> = [
  new Grip(-1, -1),
  new Grip(0, -1),
  new Grip(1, -1),
  new Grip(-1, 0),
  new Grip(1, 0),
  new Grip(-1, 1),
  new Grip(0, 1),
  new Grip(1, 1),
];

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
    return this.with(box.clipped(this.shot.bounds()), this.sketch, this.tool, this.color);
  }

  /** The session where the selection is moved but stays within the screenshot. */
  slid(by: Point): Session {
    return this.with(
      this.box.shifted(by).inside(this.shot.bounds()),
      this.sketch,
      this.tool,
      this.color,
    );
  }

  stretched(grip: Grip, to: Point): Session {
    return this.framed(grip.pulled(this.box, to));
  }

  /** The session with one more shape. A blank shape changes nothing. */
  drawn(shape: Shape): Session {
    if (shape.blank()) {
      return this;
    }
    return this.with(this.box, this.sketch.plus(shape), this.tool, this.color);
  }

  undone(): Session {
    return this.with(this.box, this.sketch.undone(), this.tool, this.color);
  }

  equipped(tool: Tool): Session {
    return this.with(this.box, this.sketch, tool, this.color);
  }

  dyed(color: string): Session {
    return this.with(this.box, this.sketch, this.tool, color);
  }

  /** True if something is drawn on the screenshot. */
  marked(): boolean {
    return !this.sketch.blank();
  }

  /** The selection. */
  place(): Box {
    return this.box;
  }

  /** The movement that starts when the mouse button is pressed at the point. */
  press(at: Point): Gesture {
    if (!this.box.blank()) {
      for (const grip of GRIPS) {
        if (grip.near(this.box, at, this.reach())) {
          return new Resizing(this, grip, at);
        }
      }
      if (this.box.holds(at)) {
        return this.tool.gesture(this, at, this.color);
      }
    }
    return new Framing(this, at, at);
  }

  /** The shape of the cursor over the point. */
  cursor(at: Point): string {
    if (this.box.blank()) {
      return 'crosshair';
    }
    for (const grip of GRIPS) {
      if (grip.near(this.box, at, this.reach())) {
        return grip.cursor();
      }
    }
    return this.box.holds(at) ? this.tool.cursor() : 'crosshair';
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
    }
  }

  cut(): Cut {
    return new Cut(this.shot, this.box, this.sketch);
  }

  private with(box: Box, sketch: Sketch, tool: Tool, color: string): Session {
    return new Session(this.shot, box, sketch, tool, color, this.unit);
  }

  /** How far from a handle a press still counts as a press on the handle. */
  private reach(): number {
    return 8 * this.unit;
  }

  private outline(ctx: CanvasRenderingContext2D): void {
    const half = 3 * this.unit;
    ctx.save();
    ctx.lineWidth = this.unit;
    ctx.strokeStyle = '#000';
    ctx.beginPath();
    this.box.trace(ctx);
    ctx.stroke();
    ctx.strokeStyle = '#fff';
    ctx.setLineDash([4 * this.unit, 4 * this.unit]);
    ctx.stroke();
    ctx.setLineDash([]);
    ctx.fillStyle = '#fff';
    ctx.strokeStyle = '#000';
    GRIPS.forEach((grip) => {
      const spot = grip.spot(this.box);
      ctx.fillRect(spot.x - half, spot.y - half, half * 2, half * 2);
      ctx.strokeRect(spot.x - half, spot.y - half, half * 2, half * 2);
    });
    ctx.restore();
  }
}

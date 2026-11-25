/** The thinnest line and the thickest one, in pixels of the window. */
export const THINNEST = 1;
export const THICKEST = 72;

/** How far the wheel has to go to change the thickness once, in pixels. */
const NOTCH = 50;

/** The thickness after the wheel has been turned by the notches: the thicker, the faster. */
export function turned(width: number, notches: number): number {
  const step = Math.max(1, Math.round(width / 10));
  return Math.min(Math.max(Math.round(width) + notches * step, THINNEST), THICKEST);
}

/** A turn of the wheel as the window reports it. WheelEvent looks like this. */
export interface Turn {
  readonly deltaY: number;
}

/** The wheel of the mouse. Small turns of a touchpad add up until they make a notch. */
export class Wheel {
  private rolled = 0;

  /** How many notches the wheel has gone toward a person. */
  notches(turn: Turn): number {
    this.rolled += turn.deltaY;
    const whole = Math.trunc(this.rolled / NOTCH);
    this.rolled -= whole * NOTCH;
    return whole;
  }
}

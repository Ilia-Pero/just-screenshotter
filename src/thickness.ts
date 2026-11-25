/** The thinnest line and the thickest one, in pixels of the window. */
export const THINNEST = 1;
export const THICKEST = 72;

/** The thickness after the wheel has been turned by the notches: the thicker, the faster. */
export function turned(width: number, notches: number): number {
  const step = Math.max(1, Math.round(width / 10));
  return Math.min(Math.max(Math.round(width) + notches * step, THINNEST), THICKEST);
}

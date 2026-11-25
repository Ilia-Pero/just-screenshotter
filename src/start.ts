import type { Box } from './box.ts';
import { brushes } from './brushes.ts';
import { RED } from './colors.ts';
import type { Desk } from './desk.ts';
import { Entry } from './entry.ts';
import { Rest } from './gesture.ts';
import { Overlay } from './overlay.ts';
import { Panels } from './panels.ts';
import { Tape } from './ruler.ts';
import { Session } from './session.ts';
import { Sketch } from './sketch.ts';
import { Hand } from './tool.ts';
import type { Words } from './words.ts';

/** A canvas of the given size that covers the window. */
function easel(root: HTMLElement, sheet: Box): CanvasRenderingContext2D {
  const canvas = document.createElement('canvas');
  canvas.className = 'view';
  canvas.width = sheet.width();
  canvas.height = sheet.height();
  root.appendChild(canvas);
  const ctx = canvas.getContext('2d');
  if (ctx === null) {
    throw new Error('The window has no canvas to draw on');
  }
  return ctx;
}

/** Takes the screenshot from the desktop and opens the editor over it. */
export function start(desk: Desk, root: HTMLElement, words: Words): Promise<void> {
  return Promise.all([desk.shot(), desk.shares(), desk.opening()]).then((ready) => {
    const shot = ready[0];
    const sheet = shot.bounds();
    const unit = sheet.width() / root.clientWidth;
    const opening = ready[2];
    const ctx = easel(root, sheet);
    new Overlay(
      desk,
      ctx,
      unit,
      new Panels(root, words, unit, ready[1], brushes(unit, new Tape(ctx))),
      new Entry(root, unit),
      new Rest(new Session(shot, opening.area, new Sketch([], []), new Hand(), RED, unit)),
      opening.film,
    ).open();
  });
}

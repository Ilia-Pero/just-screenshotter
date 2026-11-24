import type { Box } from './box.ts';
import type { Shape } from './shape.ts';
import type { Shot } from './shot.ts';

/** The selected part of the screenshot together with what is drawn on it. */
export class Cut {
  private readonly shot: Shot;
  private readonly box: Box;
  private readonly sketch: Shape;

  constructor(shot: Shot, box: Box, sketch: Shape) {
    this.shot = shot;
    this.box = box;
    this.sketch = sketch;
  }

  png(): Promise<Blob> {
    return new Promise((resolve, reject) => {
      const canvas = document.createElement('canvas');
      canvas.width = this.box.width();
      canvas.height = this.box.height();
      const ctx = canvas.getContext('2d');
      if (ctx === null) {
        reject(new Error('There is no canvas to put the picture together'));
        return;
      }
      ctx.translate(-this.box.left(), -this.box.top());
      this.shot.paint(ctx);
      this.sketch.paint(ctx);
      canvas.toBlob((blob) => {
        if (blob === null) {
          reject(new Error('The picture cannot be written as PNG'));
        } else {
          resolve(blob);
        }
      }, 'image/png');
    });
  }
}

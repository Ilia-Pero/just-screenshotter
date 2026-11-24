import { Box } from './box.ts';
import { Point } from './point.ts';
import type { Shot } from './shot.ts';

/** A screenshot loaded into a browser image. */
export class Photo implements Shot {
  private readonly image: HTMLImageElement;

  constructor(image: HTMLImageElement) {
    this.image = image;
  }

  bounds(): Box {
    return new Box(new Point(0, 0), new Point(this.image.naturalWidth, this.image.naturalHeight));
  }

  paint(ctx: CanvasRenderingContext2D): void {
    ctx.drawImage(this.image, 0, 0);
  }
}

/** A screenshot made of the image at the address. Ready when the image is loaded. */
export function developed(url: string): Promise<Shot> {
  return new Promise((resolve, reject) => {
    const image = new Image();
    image.addEventListener('load', () => resolve(new Photo(image)));
    image.addEventListener('error', () => reject(new Error('The screenshot has not loaded')));
    image.src = url;
  });
}

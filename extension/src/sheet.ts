import type { Taken } from './browser.ts';

function loaded(url: string): Promise<HTMLImageElement> {
  const image = new Image();
  image.src = url;
  return image.decode().then(() => image);
}

/** The picture as a whole. Pieces overlap, and the later ones lie over the earlier ones. */
export async function sheet(taken: Taken): Promise<HTMLCanvasElement> {
  const images = await Promise.all(taken.pieces.map((piece) => loaded(piece.url)));
  const canvas = document.createElement('canvas');
  canvas.width = Math.round(taken.width * taken.ratio);
  canvas.height = Math.round(taken.height * taken.ratio);
  const ctx = canvas.getContext('2d');
  if (ctx === null) {
    throw new Error('The browser has no canvas for the picture');
  }
  images.forEach((image, index) =>
    ctx.drawImage(image, 0, Math.round((taken.pieces[index]?.top ?? 0) * taken.ratio)),
  );
  return canvas;
}

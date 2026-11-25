/**
 * A tall page cut into strips, each as high as the window. The last one reaches the end of the
 * page, so it may cover a part of the one before it.
 */
export class Strips {
  private readonly height: number;
  private readonly view: number;

  constructor(height: number, view: number) {
    this.height = height;
    this.view = view;
  }

  /** How far to scroll for each strip. */
  tops(): ReadonlyArray<number> {
    const last = Math.max(0, this.height - this.view);
    const count = Math.ceil(last / Math.max(1, this.view)) + 1;
    return Array.from({ length: count }, (_, index) => Math.min(index * this.view, last));
  }
}

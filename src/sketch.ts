import type { Point } from './point.ts';
import type { Mark, Shape } from './shape.ts';

/**
 * The marks after one change. The reason is the same for changes that go on and on, such as
 * every notch of the thickness: they are undone together.
 */
interface Step {
  readonly reason: string;
  readonly marks: ReadonlyArray<Mark>;
}

/** Everything drawn over the screenshot: all steps of the work, and which mark is picked. */
export class Sketch implements Shape {
  private readonly steps: ReadonlyArray<Step>;
  private readonly choice: ReadonlyArray<number>;
  private readonly held: boolean;

  /**
   * choice is the number of the picked mark, or nothing. held tells whether a person has picked it:
   * a mark that has just been drawn is picked by itself.
   */
  constructor(steps: ReadonlyArray<Step>, choice: ReadonlyArray<number>, held = false) {
    this.steps = steps;
    this.choice = choice;
    this.held = held;
  }

  blank(): boolean {
    return this.marks().length === 0;
  }

  paint(ctx: CanvasRenderingContext2D): void {
    this.marks().forEach((mark) => mark.paint(ctx));
  }

  /** The sketch with one more mark over the others. */
  plus(mark: Mark): Sketch {
    return this.next('', [...this.marks(), mark], []);
  }

  /** True if there is something to undo. */
  begun(): boolean {
    return this.steps.length > 0;
  }

  /** The sketch before its last change. */
  undone(): Sketch {
    return new Sketch(this.steps.slice(0, -1), []);
  }

  /** The picked mark, or nothing. */
  picked(): ReadonlyArray<Mark> {
    return this.marks().filter((_, index) => this.chosen(index));
  }

  /** The sketch where the topmost mark at the point is picked, or none if there is none. */
  picking(at: Point, reach: number): Sketch {
    const found = this.marks()
      .map((mark, index) => (mark.touched(at, reach) ? index : -1))
      .filter((index) => index >= 0);
    return new Sketch(this.steps, found.slice(-1), true);
  }

  /** True if a person has picked a mark: one that has just been drawn is picked by itself. */
  taken(): boolean {
    return this.held && this.choice.length > 0;
  }

  /** True if a person has picked the mark at the point. */
  holding(at: Point, reach: number): boolean {
    return this.held && this.picked().some((mark) => mark.touched(at, reach));
  }

  /** The sketch where the last mark is picked. */
  latest(): Sketch {
    const last = this.marks().length - 1;
    return new Sketch(
      this.steps,
      [last].filter((index) => index >= 0),
    );
  }

  /** The sketch where no mark is picked. */
  released(): Sketch {
    return new Sketch(this.steps, []);
  }

  /** The sketch where the picked mark has changed. */
  changed(change: (mark: Mark) => Mark, reason: string): Sketch {
    return this.choice.length === 0
      ? this
      : this.next(
          reason,
          this.marks().map((mark, index) => (this.chosen(index) ? change(mark) : mark)),
          this.choice,
        );
  }

  /** The sketch without the picked mark. */
  dropped(): Sketch {
    return this.choice.length === 0
      ? this
      : this.next(
          '',
          this.marks().filter((_, index) => !this.chosen(index)),
          [],
        );
  }

  private next(
    reason: string,
    marks: ReadonlyArray<Mark>,
    choice: ReadonlyArray<number>,
  ): Sketch {
    const full = reason === '' ? '' : `${reason} ${choice.join()}`;
    const same = full !== '' && this.steps.slice(-1).some((step) => step.reason === full);
    return new Sketch(
      [...(same ? this.steps.slice(0, -1) : this.steps), { reason: full, marks }],
      choice,
      this.held && choice.length > 0,
    );
  }

  private chosen(index: number): boolean {
    return this.choice.indexOf(index) >= 0;
  }

  private marks(): ReadonlyArray<Mark> {
    return this.steps.slice(-1).reduce<ReadonlyArray<Mark>>((_, step) => step.marks, []);
  }
}

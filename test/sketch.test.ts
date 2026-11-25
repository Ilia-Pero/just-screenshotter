import assert from 'node:assert/strict';
import { test } from 'node:test';
import { Segment } from '../src/segment.ts';
import { Sketch } from '../src/sketch.ts';
import { ink, point } from './fakes.ts';

const empty = new Sketch([], []);

/** A line along the top from the left edge, as long as given. */
function line(length: number): Segment {
  return new Segment(point(0, 0), point(length, 0), ink(1));
}

/** Thicknesses of the picked mark: one or none. */
function widths(sketch: Sketch): ReadonlyArray<number> {
  return sketch.picked().map((mark) => mark.tint().times(1));
}

/** The sketch where the picked mark has the thickness. */
function thick(sketch: Sketch, width: number): Sketch {
  return sketch.changed((mark) => mark.tinted(mark.tint().thick(width)), 'thick');
}

test('a sketch without marks is blank', () => {
  assert.equal(empty.blank(), true);
});

test('a sketch with a mark is not blank', () => {
  assert.equal(empty.plus(line(5)).blank(), false);
});

test('undo removes the last mark', () => {
  assert.equal(empty.plus(line(5)).undone().blank(), true);
});

test('undo in a blank sketch leaves it blank', () => {
  assert.equal(empty.undone().blank(), true);
});

test('there is nothing to undo at first', () => {
  assert.equal(empty.begun(), false);
});

test('a mark under the point is picked', () => {
  assert.equal(empty.plus(line(50)).picking(point(20, 3), 5).picked().length, 1);
});

test('of two marks under the point, the upper one is picked', () => {
  const upper = line(50);
  assert.equal(empty.plus(line(50)).plus(upper).picking(point(20, 0), 5).picked()[0], upper);
});

test('nothing is picked far from the marks', () => {
  assert.equal(empty.plus(line(50)).latest().picking(point(20, 30), 5).picked().length, 0);
});

test('the last mark of a blank sketch is none', () => {
  assert.equal(empty.latest().picked().length, 0);
});

test('a change touches the picked mark', () => {
  assert.deepEqual(widths(thick(empty.plus(line(50)).latest(), 7)), [7]);
});

test('a change does not touch the other marks', () => {
  assert.deepEqual(
    widths(thick(empty.plus(line(50)).plus(line(80)).latest(), 7).picking(point(40, 0), 0)),
    [7],
  );
});

test('a change without a picked mark adds no step', () => {
  assert.equal(thick(empty.plus(line(50)), 7).undone().blank(), true);
});

test('several notches of the thickness in a row are undone together', () => {
  assert.deepEqual(
    widths(thick(thick(empty.plus(line(50)).latest(), 7), 9).undone().latest()),
    [1],
  );
});

test('a notch after another change is undone separately', () => {
  const thicker = thick(empty.plus(line(50)).latest(), 7);
  const moved = thicker.changed((mark) => mark.shifted(point(1, 1)), '');
  assert.deepEqual(widths(thick(moved, 9).undone().latest()), [7]);
});

test('the picked mark is deleted', () => {
  assert.equal(empty.plus(line(50)).latest().dropped().blank(), true);
});

test('a deleted mark comes back when undone', () => {
  assert.equal(empty.plus(line(50)).latest().dropped().undone().blank(), false);
});

test('nothing is deleted when nothing is picked', () => {
  assert.equal(empty.plus(line(50)).dropped().blank(), false);
});

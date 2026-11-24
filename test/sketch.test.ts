import assert from 'node:assert/strict';
import { test } from 'node:test';
import { Segment } from '../src/segment.ts';
import { Sketch } from '../src/sketch.ts';
import { ink, point } from './fakes.ts';

test('a sketch without shapes is blank', () => {
  assert.equal(new Sketch([]).blank(), true);
});

test('a sketch with a shape is not blank', () => {
  assert.equal(new Sketch([]).plus(new Segment(point(0, 0), point(5, 5), ink(1))).blank(), false);
});

test('undo removes the last shape', () => {
  assert.equal(
    new Sketch([]).plus(new Segment(point(0, 0), point(5, 5), ink(1))).undone().blank(),
    true,
  );
});

test('undo in a blank sketch leaves it blank', () => {
  assert.equal(new Sketch([]).undone().blank(), true);
});

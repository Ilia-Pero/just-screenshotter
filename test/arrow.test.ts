import assert from 'node:assert/strict';
import { test } from 'node:test';
import { Arrow } from '../src/arrow.ts';
import { ink, point } from './fakes.ts';

test('the tip of the head is the end of the arrow', () => {
  assert.deepEqual(new Arrow(point(0, 0), point(100, 0), ink(2)).head()[0], point(100, 0));
});

test('the base of the head is five line widths away from the end', () => {
  assert.deepEqual(new Arrow(point(0, 0), point(100, 0), ink(2)).head()[1], point(90, 4.5));
});

test('the corners of the base lie on both sides of the line', () => {
  assert.deepEqual(new Arrow(point(0, 0), point(100, 0), ink(2)).head()[2], point(90, -4.5));
});

test('the head of a short arrow is not longer than the arrow', () => {
  assert.deepEqual(new Arrow(point(0, 0), point(4, 0), ink(2)).head()[1], point(0, 1.8));
});

test('an arrow of zero length is blank', () => {
  assert.equal(new Arrow(point(7, 7), point(7, 7), ink(2)).blank(), true);
});

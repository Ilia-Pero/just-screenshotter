import assert from 'node:assert/strict';
import { test } from 'node:test';
import { box, point } from './fakes.ts';

test('the left side is the smaller coordinate, whatever the order of the corners', () => {
  assert.equal(box(10, 10, 2, 4).left(), 2);
});

test('the height is never negative, even if the corners are swapped', () => {
  assert.equal(box(10, 10, 2, 4).height(), 6);
});

test('a rectangle of zero width is blank', () => {
  assert.equal(box(5, 0, 5, 30).blank(), true);
});

test('a rectangle of one pixel is not blank', () => {
  assert.equal(box(5, 5, 6, 6).blank(), false);
});

test('a point on the border belongs to the rectangle', () => {
  assert.equal(box(0, 0, 10, 10).holds(point(10, 3)), true);
});

test('a point beyond the border does not belong to the rectangle', () => {
  assert.equal(box(0, 0, 10, 10).holds(point(11, 3)), false);
});

test('a shift moves the right side', () => {
  assert.equal(box(0, 0, 10, 10).shifted(point(5, 0)).right(), 15);
});

test('scaling multiplies the width', () => {
  assert.equal(box(2, 2, 12, 12).scaled(0.5).width(), 5);
});

test('a rectangle sticking out to the right goes back to the right side of the frame', () => {
  assert.equal(box(95, 0, 115, 10).inside(box(0, 0, 100, 80)).left(), 80);
});

test('a rectangle sticking out of the top goes back to the top side of the frame', () => {
  assert.equal(box(0, -7, 10, 3).inside(box(0, 0, 100, 80)).bottom(), 10);
});

test('clipping removes the part beyond the frame', () => {
  assert.equal(box(90, 0, 130, 10).clipped(box(0, 0, 100, 80)).width(), 10);
});

test('a rectangle wholly beyond the frame is blank after clipping', () => {
  assert.equal(box(120, 0, 130, 10).clipped(box(0, 0, 100, 80)).blank(), true);
});

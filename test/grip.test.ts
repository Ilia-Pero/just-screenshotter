import assert from 'node:assert/strict';
import { test } from 'node:test';
import { Grip } from '../src/grip.ts';
import { box, point } from './fakes.ts';

test('the bottom right handle sits in the bottom right corner', () => {
  assert.deepEqual(new Grip(1, 1).spot(box(10, 10, 60, 40)), point(60, 40));
});

test('the top middle handle sits in the middle of the top side', () => {
  assert.deepEqual(new Grip(0, -1).spot(box(10, 10, 60, 40)), point(35, 10));
});

test('a handle responds to a press next to it', () => {
  assert.equal(new Grip(1, 1).near(box(10, 10, 60, 40), point(63, 44), 8), true);
});

test('a handle does not respond to a press far from it', () => {
  assert.equal(new Grip(1, 1).near(box(10, 10, 60, 40), point(35, 25), 8), false);
});

test('the right handle moves nothing but the right side', () => {
  assert.equal(new Grip(1, 0).pulled(box(10, 10, 60, 40), point(80, 70)).height(), 30);
});

test('the left handle dragged past the right side turns the rectangle over', () => {
  assert.equal(new Grip(-1, 0).pulled(box(10, 10, 60, 40), point(90, 20)).left(), 60);
});

test('a corner handle on the main diagonal shows a diagonal cursor', () => {
  assert.equal(new Grip(-1, -1).cursor(), 'nwse-resize');
});

test('a side handle shows a horizontal cursor', () => {
  assert.equal(new Grip(1, 0).cursor(), 'ew-resize');
});

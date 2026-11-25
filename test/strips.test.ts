import assert from 'node:assert/strict';
import { test } from 'node:test';
import { Strips } from '../extension/src/strips.ts';

test('a page that fits into the window is one strip', () => {
  assert.deepEqual(new Strips(800, 800).tops(), [0]);
});

test('a page lower than the window is one strip', () => {
  assert.deepEqual(new Strips(300, 800).tops(), [0]);
});

test('strips follow one another', () => {
  assert.deepEqual(new Strips(2400, 800).tops(), [0, 800, 1600]);
});

test('the last strip reaches the end of the page', () => {
  assert.deepEqual(new Strips(2000, 800).tops(), [0, 800, 1200]);
});

test('a window without a height does not cut for ever', () => {
  assert.deepEqual(new Strips(0, 0).tops(), [0]);
});

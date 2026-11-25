import assert from 'node:assert/strict';
import { test } from 'node:test';
import { THICKEST, THINNEST, turned } from '../src/thickness.ts';

test('a notch makes a thin line thicker by one pixel', () => {
  assert.equal(turned(3, 1), 4);
});

test('a notch changes a thick line faster', () => {
  assert.equal(turned(40, -1), 36);
});

test('a line does not vanish', () => {
  assert.equal(turned(2, -20), THINNEST);
});

test('a line does not grow without a limit', () => {
  assert.equal(turned(60, 20), THICKEST);
});

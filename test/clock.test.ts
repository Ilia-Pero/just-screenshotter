import assert from 'node:assert/strict';
import { test } from 'node:test';
import { clock } from '../src/clock.ts';

test('a film begins at zero', () => {
  assert.equal(clock(0), '00:00');
});

test('a part of a second is not counted', () => {
  assert.equal(clock(61_999), '01:01');
});

test('hours appear when there are any', () => {
  assert.equal(clock(3_600_000 + 5 * 60_000 + 7000), '1:05:07');
});

test('a clock that is behind shows zero', () => {
  assert.equal(clock(-5000), '00:00');
});

import assert from 'node:assert/strict';
import { test } from 'node:test';
import { THICKEST, THINNEST, Wheel, turned } from '../src/thickness.ts';

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

test('a turn of a mouse wheel is two notches', () => {
  assert.equal(new Wheel().notches({ deltaY: -100 }), -2);
});

test('a small turn is not a notch yet', () => {
  assert.equal(new Wheel().notches({ deltaY: 30 }), 0);
});

test('small turns add up to a notch', () => {
  const wheel = new Wheel();
  wheel.notches({ deltaY: 30 });
  assert.equal(wheel.notches({ deltaY: 30 }), 1);
});

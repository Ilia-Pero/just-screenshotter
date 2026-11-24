import assert from 'node:assert/strict';
import { test } from 'node:test';
import { Key } from '../src/key.ts';

test('a key is recognised by its name', () => {
  assert.equal(new Key('Escape', 27).struck({ code: 'Escape', keyCode: 27 }), true);
});

test('a key is recognised by its name in a layout with another number', () => {
  assert.equal(new Key('KeyC', 67).struck({ code: 'KeyC', keyCode: 1057 }), true);
});

test('a nameless press is recognised by its number', () => {
  assert.equal(new Key('Escape', 27).struck({ code: '', keyCode: 27 }), true);
});

test('another named key does not fit, even with the same number', () => {
  assert.equal(new Key('Enter', 13).struck({ code: 'NumpadEnter', keyCode: 13 }), false);
});

test('another nameless key does not fit', () => {
  assert.equal(new Key('Escape', 27).struck({ code: '', keyCode: 65 }), false);
});

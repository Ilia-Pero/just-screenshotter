import assert from 'node:assert/strict';
import { test } from 'node:test';
import type { Combination } from '../src/key.ts';
import { Key, Letter, browsing } from '../src/key.ts';

/** A press of the key with Ctrl held. */
function held(code: string, key: string): Combination {
  return { code, key, keyCode: 0, ctrlKey: true, altKey: false, metaKey: false };
}

test('a key is recognised by its name', () => {
  assert.equal(new Key('Escape', 27).struck({ code: 'Escape', key: 'Escape', keyCode: 27 }), true);
});

test('a key is recognised by its name in a layout with another number', () => {
  assert.equal(new Key('KeyC', 67).struck({ code: 'KeyC', key: 'с', keyCode: 1057 }), true);
});

test('a nameless press is recognised by its number', () => {
  assert.equal(new Key('Escape', 27).struck({ code: '', key: '', keyCode: 27 }), true);
});

test('another named key does not fit, even with the same number', () => {
  assert.equal(new Key('Enter', 13).struck({ code: 'NumpadEnter', key: 'Enter', keyCode: 13 }), false);
});

test('another nameless key does not fit', () => {
  assert.equal(new Key('Escape', 27).struck({ code: '', key: '', keyCode: 65 }), false);
});

test('a letter is recognised by what is written on the key', () => {
  assert.equal(new Letter('Z').struck({ code: 'KeyY', key: 'z', keyCode: 90 }), true);
});

test('a key in the place of the letter is not the letter', () => {
  assert.equal(new Letter('Z').struck({ code: 'KeyZ', key: 'y', keyCode: 89 }), false);
});

test('a letter is recognised when it is a capital', () => {
  assert.equal(new Letter('C').struck({ code: 'KeyC', key: 'C', keyCode: 67 }), true);
});

test('a letter is recognised by its place in another alphabet', () => {
  assert.equal(new Letter('Z').struck({ code: 'KeyZ', key: 'я', keyCode: 90 }), true);
});

test('a browser loads the page anew with F5', () => {
  assert.equal(browsing({ ...held('F5', 'F5'), ctrlKey: false }), true);
});

test('a browser prints with Ctrl and P', () => {
  assert.equal(browsing(held('KeyP', 'p')), true);
});

test('a browser prints with Ctrl and P in another alphabet', () => {
  assert.equal(browsing(held('KeyP', 'з')), true);
});

test('a text is pasted with Ctrl and V', () => {
  assert.equal(browsing(held('KeyV', 'v')), false);
});

test('a text is read word by word with Ctrl and an arrow', () => {
  assert.equal(browsing(held('ArrowLeft', 'ArrowLeft')), false);
});

test('a sign is typed with Ctrl and Alt', () => {
  assert.equal(browsing({ ...held('KeyQ', '@'), altKey: true }), false);
});

test('a window is closed with Alt and F4', () => {
  assert.equal(browsing({ ...held('F4', 'F4'), ctrlKey: false, altKey: true }), false);
});

test('a letter alone is typed', () => {
  assert.equal(browsing({ ...held('KeyR', 'r'), ctrlKey: false }), false);
});

import assert from 'node:assert/strict';
import { test } from 'node:test';
import type { Struck } from '../src/chord.ts';
import { Chord } from '../src/chord.ts';

/** A press of the key alone. */
function struck(code: string): Struck {
  return { code, key: '', ctrlKey: false, shiftKey: false, altKey: false, metaKey: false };
}

test('a letter is spelled without the word key', () => {
  assert.equal(new Chord({ ...struck('KeyS'), ctrlKey: true }, false).spelled(), 'Ctrl+S');
});

test('held keys are spelled in the same order', () => {
  assert.equal(
    new Chord({ ...struck('Digit9'), shiftKey: true, ctrlKey: true, altKey: true }, false).spelled(),
    'Ctrl+Alt+Shift+9',
  );
});

test('a key without a place is known by its name', () => {
  assert.equal(new Chord({ ...struck(''), key: 'PrintScreen' }, false).spelled(), 'PrintScreen');
});

test('a held key alone is not the end', () => {
  assert.equal(new Chord({ ...struck('ControlLeft'), ctrlKey: true }, false).pending(), true);
});

test('a key pressed with a held one is the end', () => {
  assert.equal(new Chord({ ...struck('KeyS'), ctrlKey: true }, false).pending(), false);
});

test('Print Screen may be pressed alone', () => {
  assert.equal(new Chord(struck('PrintScreen'), false).fit(), true);
});

test('a function key may be pressed alone', () => {
  assert.equal(new Chord(struck('F9'), false).fit(), true);
});

test('a letter alone gets in the way of typing', () => {
  assert.equal(new Chord(struck('KeyS'), false).fit(), false);
});

test('a letter with Shift gets in the way of typing', () => {
  assert.equal(new Chord({ ...struck('KeyS'), shiftKey: true }, false).fit(), false);
});

test('a letter with Ctrl does not get in the way', () => {
  assert.equal(new Chord({ ...struck('KeyS'), ctrlKey: true }, false).fit(), true);
});

test('a key the shell does not know is not fit', () => {
  assert.equal(new Chord({ ...struck('MediaPlayPause'), ctrlKey: true }, false).fit(), false);
});

test('a letter is known by what is written on the key', () => {
  assert.equal(new Chord({ ...struck('KeyY'), key: 'z', ctrlKey: true }, false).spelled(), 'Ctrl+Z');
});

test('a letter is known by its place where the system knows it so', () => {
  assert.equal(new Chord({ ...struck('KeyY'), key: 'z', ctrlKey: true }, true).spelled(), 'Ctrl+Y');
});

test('a letter of another alphabet is known by its place', () => {
  assert.equal(new Chord({ ...struck('KeyS'), key: 'ы', ctrlKey: true }, false).spelled(), 'Ctrl+S');
});

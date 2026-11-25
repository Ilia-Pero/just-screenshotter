import assert from 'node:assert/strict';
import { test } from 'node:test';
import type { Struck } from '../src/chord.ts';
import { Chord } from '../src/chord.ts';

/** A press of the key alone. */
function struck(code: string): Struck {
  return { code, key: '', ctrlKey: false, shiftKey: false, altKey: false, metaKey: false };
}

test('a letter is spelled without the word key', () => {
  assert.equal(new Chord({ ...struck('KeyS'), ctrlKey: true }).spelled(), 'Ctrl+S');
});

test('held keys are spelled in the same order', () => {
  assert.equal(
    new Chord({ ...struck('Digit9'), shiftKey: true, ctrlKey: true, altKey: true }).spelled(),
    'Ctrl+Alt+Shift+9',
  );
});

test('a key without a place is known by its name', () => {
  assert.equal(new Chord({ ...struck(''), key: 'PrintScreen' }).spelled(), 'PrintScreen');
});

test('a held key alone is not the end', () => {
  assert.equal(new Chord({ ...struck('ControlLeft'), ctrlKey: true }).pending(), true);
});

test('a key pressed with a held one is the end', () => {
  assert.equal(new Chord({ ...struck('KeyS'), ctrlKey: true }).pending(), false);
});

test('Print Screen may be pressed alone', () => {
  assert.equal(new Chord(struck('PrintScreen')).fit(), true);
});

test('a function key may be pressed alone', () => {
  assert.equal(new Chord(struck('F9')).fit(), true);
});

test('a letter alone gets in the way of typing', () => {
  assert.equal(new Chord(struck('KeyS')).fit(), false);
});

test('a letter with Shift gets in the way of typing', () => {
  assert.equal(new Chord({ ...struck('KeyS'), shiftKey: true }).fit(), false);
});

test('a letter with Ctrl does not get in the way', () => {
  assert.equal(new Chord({ ...struck('KeyS'), ctrlKey: true }).fit(), true);
});

test('a key the shell does not know is not fit', () => {
  assert.equal(new Chord({ ...struck('MediaPlayPause'), ctrlKey: true }).fit(), false);
});

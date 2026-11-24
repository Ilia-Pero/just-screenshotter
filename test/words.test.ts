import assert from 'node:assert/strict';
import { test } from 'node:test';
import { spoken } from '../src/languages.ts';
import { filled } from '../src/words.ts';

test('a system is spoken to in its own language', () => {
  assert.equal(spoken('ru-RU').pencil, 'Карандаш');
});

test('a tag with a script is understood', () => {
  assert.equal(spoken('zh-Hans-CN').pencil, '铅笔');
});

test('an unknown language falls back to English', () => {
  assert.equal(spoken('fi-FI').pencil, 'Pencil');
});

test('a language that merely starts with the same letters is not taken for it', () => {
  assert.equal(spoken('run').pencil, 'Pencil');
});

test('a hole in the text is filled', () => {
  assert.equal(filled('It did not work: {reason}', 'reason', 'no disk'), 'It did not work: no disk');
});

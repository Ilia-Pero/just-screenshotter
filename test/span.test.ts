import assert from 'node:assert/strict';
import { test } from 'node:test';
import { Span } from '../src/span.ts';
import { point } from './fakes.ts';

test('the distance to a point beside the stretch is measured across it', () => {
  assert.equal(new Span(point(0, 0), point(100, 0)).distance(point(40, 7)), 7);
});

test('the distance to a point beyond the end is measured to the end', () => {
  assert.equal(new Span(point(0, 0), point(100, 0)).distance(point(103, 4)), 5);
});

test('a stretch of zero length is a point', () => {
  assert.equal(new Span(point(1, 1), point(1, 1)).distance(point(4, 5)), 5);
});

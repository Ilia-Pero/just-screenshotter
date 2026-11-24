import assert from 'node:assert/strict';
import { test } from 'node:test';
import type { Box } from '../src/box.ts';
import { Layout } from '../src/layout.ts';
import { box, point } from './fakes.ts';

/** Panels around the area in a window of 1000 by 800. */
function layout(area: Box): Layout {
  return new Layout(
    { width: 1000, height: 800 },
    area,
    { width: 40, height: 260 },
    { width: 140, height: 40 },
    { width: 80, height: 20 },
  );
}

test('the tools stand to the right of the selection, level with its bottom', () => {
  assert.deepEqual(layout(box(200, 200, 600, 600)).toolsSpot(), point(608, 340));
});

test('the tools go inside a selection that touches the right edge', () => {
  assert.deepEqual(layout(box(200, 200, 1000, 600)).toolsSpot(), point(952, 340));
});

test('the actions stand below the selection, level with its right side', () => {
  assert.deepEqual(layout(box(200, 200, 600, 600)).actionsSpot(), point(460, 608));
});

test('the actions go above a selection that touches the bottom edge', () => {
  assert.deepEqual(layout(box(200, 200, 600, 800)).actionsSpot(), point(460, 152));
});

test('the actions go inside a selection as high as the window', () => {
  assert.deepEqual(layout(box(200, 0, 600, 800)).actionsSpot(), point(460, 752));
});

test('the actions leave room for the tools that went inside the selection', () => {
  assert.deepEqual(layout(box(200, 200, 1000, 600)).actionsSpot(), point(804, 608));
});

test('the size stands above the selection', () => {
  assert.deepEqual(layout(box(200, 200, 600, 600)).badgeSpot(), point(200, 172));
});

test('the size goes inside a selection that touches the top edge', () => {
  assert.deepEqual(layout(box(200, 0, 600, 600)).badgeSpot(), point(200, 8));
});

test('the tools of a tiny selection in the corner stay on the screen', () => {
  assert.deepEqual(layout(box(0, 0, 10, 10)).toolsSpot(), point(18, 8));
});

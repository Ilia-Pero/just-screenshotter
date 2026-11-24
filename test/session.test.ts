import assert from 'node:assert/strict';
import { test } from 'node:test';
import { Caption } from '../src/caption.ts';
import { Segment } from '../src/segment.ts';
import { Stroke } from '../src/stroke.ts';
import { Brush, Hand } from '../src/tool.ts';
import { dragged, framed, fresh, point } from './fakes.ts';

const pencil = new Brush(3, 1, 'crosshair', (at, ink) => new Stroke([at], ink));
const line = new Brush(3, 1, 'crosshair', (at, ink) => new Segment(at, at, ink));
const text = new Brush(1, 1, 'text', (at, ink) => new Caption(at, ink, 18));

test('a drag over the screenshot selects an area of the same width', async () => {
  assert.equal((await framed(new Hand())).place().width(), 50);
});

test('the selection is clipped by the edge of the screenshot', async () => {
  assert.equal(
    (await dragged(fresh(new Hand()), point(10, 10), point(500, 40), '')).place().right(),
    100,
  );
});

test('a click without a drag selects nothing', async () => {
  assert.equal(
    (await dragged(fresh(new Hand()), point(10, 10), point(10, 10), '')).place().blank(),
    true,
  );
});

test('without a tool, a drag inside the selection moves it', async () => {
  assert.equal(
    (await dragged(await framed(new Hand()), point(20, 20), point(30, 25), '')).place().left(),
    20,
  );
});

test('the selection cannot be dragged beyond the edge of the screenshot', async () => {
  assert.equal(
    (await dragged(await framed(new Hand()), point(20, 20), point(900, 20), '')).place().right(),
    100,
  );
});

test('a drag of a corner handle resizes the selection', async () => {
  assert.equal(
    (await dragged(await framed(new Hand()), point(60, 40), point(70, 60), '')).place().height(),
    50,
  );
});

test('a drag outside the selection starts a new one', async () => {
  assert.equal(
    (await dragged(await framed(new Hand()), point(70, 50), point(90, 70), '')).place().left(),
    70,
  );
});

test('a pencil stroke inside the selection stays on the screenshot', async () => {
  assert.equal(
    (await dragged(await framed(pencil), point(20, 20), point(30, 30), '')).marked(),
    true,
  );
});

test('the pencil does not move the selection', async () => {
  assert.equal(
    (await dragged(await framed(pencil), point(20, 20), point(30, 30), '')).place().left(),
    10,
  );
});

test('undo removes the only stroke', async () => {
  assert.equal(
    (await dragged(await framed(pencil), point(20, 20), point(30, 30), '')).undone().marked(),
    false,
  );
});

test('a click with the line tool draws nothing', async () => {
  assert.equal(
    (await dragged(await framed(line), point(20, 20), point(20, 20), '')).marked(),
    false,
  );
});

test('a label with some text stays on the screenshot', async () => {
  assert.equal(
    (await dragged(await framed(text), point(20, 20), point(20, 20), 'Hello')).marked(),
    true,
  );
});

test('a label of spaces does not get on the screenshot', async () => {
  assert.equal(
    (await dragged(await framed(text), point(20, 20), point(20, 20), '   ')).marked(),
    false,
  );
});

test('the cursor over a corner handle shows a diagonal resize', async () => {
  assert.equal((await framed(new Hand())).cursor(point(60, 40)), 'nwse-resize');
});

test('the cursor inside the selection shows a move when there is no tool', async () => {
  assert.equal((await framed(new Hand())).cursor(point(35, 25)), 'move');
});

test('the cursor outside the selection stays a crosshair', async () => {
  assert.equal((await framed(new Hand())).cursor(point(90, 70)), 'crosshair');
});

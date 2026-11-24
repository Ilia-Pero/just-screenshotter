import assert from 'node:assert/strict';
import { test } from 'node:test';
import { Arrow } from '../src/arrow.ts';
import { Caption } from '../src/caption.ts';
import { Segment } from '../src/segment.ts';
import type { Session } from '../src/session.ts';
import { Stroke } from '../src/stroke.ts';
import { Brush, Hand, Quill } from '../src/tool.ts';
import { Even, Script, dragged, framed, fresh, point } from './fakes.ts';

const pencil = new Brush(3, 1, (at, ink) => new Stroke([at], ink));
const line = new Brush(3, 1, (at, ink) => new Segment(at, at, ink));
const arrow = new Brush(3, 1, (at, ink) => new Arrow(at, at, ink));
const text = new Quill(new Brush(10, 1, (at, ink) => new Caption(at, ink, new Even())));

/** A session with an arrow from (20, 20) to (40, 20), which is picked. */
async function pointed(): Promise<Session> {
  return dragged(await framed(arrow), point(20, 20), point(40, 20), '');
}

/** The same session where nothing is picked and there is no tool. */
async function handed(): Promise<Session> {
  return (await pointed()).equipped(new Hand());
}

/** A session with the text "Hello" at (20, 20), which is picked. */
async function labelled(): Promise<Session> {
  return dragged(await framed(text), point(20, 20), point(20, 20), 'Hello');
}

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

test('the cursor over a mark shows that it can be picked', async () => {
  assert.equal((await handed()).cursor(point(30, 20)), 'pointer');
});

test('an arrow is picked as soon as it is drawn', async () => {
  assert.equal((await pointed()).picked(), true);
});

test('a freehand line is not picked when it is drawn', async () => {
  assert.equal(
    (await dragged(await framed(pencil), point(20, 20), point(30, 30), '')).picked(),
    false,
  );
});

test('a click on a mark picks it when there is no tool', async () => {
  assert.equal(
    (await dragged(await handed(), point(30, 20), point(30, 20), '')).picked(),
    true,
  );
});

test('a click beside the marks picks nothing', async () => {
  assert.equal(
    (await dragged(await handed(), point(30, 35), point(30, 35), '')).picked(),
    false,
  );
});

test('a drag of a mark does not move the selection', async () => {
  assert.equal(
    (await dragged(await handed(), point(30, 20), point(35, 25), '')).place().left(),
    10,
  );
});

test('a drag of a mark moves the mark', async () => {
  assert.equal(
    (await dragged(await handed(), point(30, 20), point(30, 33), '')).cursor(point(30, 33)),
    'pointer',
  );
});

test('the end of the arrow just drawn can be dragged with the same tool', async () => {
  const turned = await dragged(await pointed(), point(40, 20), point(20, 38), '');
  assert.equal(turned.equipped(new Hand()).cursor(point(20, 30)), 'pointer');
});

test('a drag of the end of an arrow is undone at once', async () => {
  const turned = await dragged(await pointed(), point(40, 20), point(20, 38), '');
  assert.equal(turned.undone().equipped(new Hand()).cursor(point(30, 20)), 'pointer');
});

test('a click on the end of an arrow adds nothing to undo', async () => {
  const clicked = await dragged(await pointed(), point(40, 20), point(40, 20), '');
  assert.equal(clicked.undone().marked(), false);
});

test('a press elsewhere draws the next arrow', async () => {
  const second = await dragged(await pointed(), point(20, 34), point(40, 34), '');
  assert.equal(second.undone().marked(), true);
});

test('the thickness of the picked mark changes', async () => {
  assert.equal((await pointed()).thick(9).thickness(), 9);
});

test('a change of the picked mark leaves the tool as it was', async () => {
  assert.equal((await pointed()).thick(9).released().thickness(), 3);
});

test('without a picked mark, the thickness of the tool changes', async () => {
  assert.equal((await framed(arrow)).thick(9).thickness(), 9);
});

test('there is no thickness without a tool and a picked mark', async () => {
  assert.equal((await framed(new Hand())).thickness(), 0);
});

test('the picked mark takes another colour', async () => {
  assert.equal((await pointed()).dyed('#00ff00').hue(), '#00ff00');
});

test('a colour given to a mark stays for the next ones', async () => {
  assert.equal((await pointed()).dyed('#00ff00').released().hue(), '#00ff00');
});

test('undo returns the colour of the mark', async () => {
  const dyed = (await pointed()).dyed('#00ff00').undone().equipped(new Hand());
  assert.equal((await dragged(dyed, point(30, 20), point(30, 20), '')).hue(), '#000000');
});

test('the picked mark is deleted', async () => {
  assert.equal((await pointed()).dropped().marked(), false);
});

test('a deleted mark can be brought back', async () => {
  assert.equal((await pointed()).dropped().undone().marked(), true);
});

test('a click on a text with the text tool changes the text', async () => {
  assert.equal(
    (await dragged(await labelled(), point(25, 25), point(25, 25), '')).marked(),
    false,
  );
});

test('a text typed anew does not become a second one', async () => {
  const changed = await dragged(await labelled(), point(25, 25), point(25, 25), 'Bye');
  assert.equal(changed.dropped().marked(), false);
});

test('a picked text is typed anew by a key', async () => {
  assert.equal(
    (await (await labelled()).rewrite().ended(new Script(''))).marked(),
    false,
  );
});

test('a key does nothing to a picked arrow', async () => {
  assert.equal((await (await pointed()).rewrite().ended(new Script(''))).marked(), true);
});

test('a text left as it was adds nothing to undo', async () => {
  const same = await (await labelled()).rewrite().ended(new Script('Hello'));
  assert.equal(same.undone().marked(), false);
});

test('a new selection leaves the mark that was picked', async () => {
  assert.equal(
    (await dragged(await pointed(), point(70, 50), point(90, 70), '')).picked(),
    false,
  );
});

test('the cursor outside the selection stays a crosshair', async () => {
  assert.equal((await framed(new Hand())).cursor(point(90, 70)), 'crosshair');
});

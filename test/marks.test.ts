import assert from 'node:assert/strict';
import { test } from 'node:test';
import { Arrow } from '../src/arrow.ts';
import { Border, Oval } from '../src/form.ts';
import { Frame } from '../src/frame.ts';
import { Label } from '../src/label.ts';
import { Stroke } from '../src/stroke.ts';
import { Even, Script, Sized, ink, point } from './fakes.ts';

const arrow = new Arrow(point(0, 0), point(100, 0), ink(2));
const frame = new Frame(point(10, 10), point(60, 40), ink(2), new Border());
const oval = new Frame(point(10, 10), point(60, 40), ink(2), new Oval());
const label = new Label(point(10, 10), 'Hello', ink(20), new Even());
const stroke = new Stroke([point(0, 0), point(10, 0), point(10, 30)], ink(2));

test('the tail of an arrow is pulled by the first handle', () => {
  assert.deepEqual(arrow.pulled(0, point(50, 50)).handles()[0], point(50, 50));
});

test('the tip of an arrow is pulled by the second handle', () => {
  assert.deepEqual(arrow.pulled(1, point(0, 70)).handles()[1], point(0, 70));
});

test('the other end of an arrow stays when one is pulled', () => {
  assert.deepEqual(arrow.pulled(1, point(0, 70)).handles()[0], point(0, 0));
});

test('a thick line is touched farther from its middle', () => {
  assert.equal(arrow.tinted(ink(20)).touched(point(50, 14), 5), true);
});

test('a thin line is not touched there', () => {
  assert.equal(arrow.touched(point(50, 14), 5), false);
});

test('a frame is touched on its line', () => {
  assert.equal(frame.touched(point(35, 11), 3), true);
});

test('the inside of a frame does not belong to it', () => {
  assert.equal(frame.touched(point(35, 25), 3), false);
});

test('a frame has a handle on every side and corner', () => {
  assert.equal(frame.handles().length, 8);
});

test('a frame is resized by its corner', () => {
  assert.equal(frame.pulled(7, point(90, 80), false).bounds().width(), 80);
});

test('a frame stays as it is if there is no such handle', () => {
  assert.equal(frame.pulled(99, point(90, 80), false), frame);
});

test('a corner with Shift makes a frame a square', () => {
  assert.equal(frame.pulled(7, point(90, 80), true).bounds().height(), 80);
});

test('the opposite corner stays when a frame becomes a square', () => {
  assert.equal(frame.pulled(0, point(0, 0), true).bounds().right(), 60);
});

test('a side with Shift resizes a frame as it does without', () => {
  assert.equal(frame.pulled(4, point(90, 80), true).bounds().height(), 30);
});

test('a frame drawn with Shift is a square', async () => {
  const drawn = await frame.extended(point(30, 50), true).settled(new Script(''));
  assert.equal(drawn.bounds().right(), 50);
});

test('an oval is touched on its line', () => {
  assert.equal(oval.touched(point(35, 11), 3), true);
});

test('the corner of the rectangle around an oval does not belong to it', () => {
  assert.equal(oval.touched(point(11, 11), 3), false);
});

test('the inside of an oval does not belong to it', () => {
  assert.equal(oval.touched(point(35, 25), 3), false);
});

test('an oval is touched between its top and its side', () => {
  assert.equal(
    oval.touched(point(35 + 25 * Math.SQRT1_2, 25 - 15 * Math.SQRT1_2), 1),
    true,
  );
});

test('an oval stays an oval when it is moved', () => {
  assert.equal(oval.shifted(point(5, 0)).touched(point(16, 11), 3), false);
});

test('an oval stays an oval when it is resized', () => {
  assert.equal(oval.pulled(7, point(90, 80), false).touched(point(89, 79), 3), false);
});

test('an oval stays an oval when its thickness changes', () => {
  assert.equal(oval.tinted(ink(4)).touched(point(11, 11), 3), false);
});

test('a freehand line is touched between its points', () => {
  assert.equal(stroke.touched(point(11, 15), 3), true);
});

test('a freehand line moves as a whole', () => {
  assert.equal(stroke.shifted(point(5, 7)).bounds().bottom(), 37);
});

test('a text is touched anywhere on its line', () => {
  assert.equal(label.touched(point(55, 30), 0), true);
});

test('a text is as wide as its letters', () => {
  assert.equal(label.bounds().width(), 50);
});

test('a text with larger letters is wider', () => {
  assert.equal(label.tinted(ink(40)).bounds().width(), 100);
});

test('a text typed anew takes the place of the old one', async () => {
  assert.equal((await label.retyped(new Script('Hi'))).bounds().width(), 20);
});

test('a text left as it was is the same mark', async () => {
  assert.equal(await label.retyped(new Script('Hello')), label);
});

test('a text typed with larger letters is wider', async () => {
  assert.equal((await label.retyped(new Sized('Hello', 40))).bounds().width(), 100);
});

test('a text of two lines is twice as tall', () => {
  assert.equal(
    new Label(point(0, 0), 'One\nTwo', ink(20), new Even()).bounds().height(),
    48,
  );
});

test('a text is as wide as its widest line', () => {
  assert.equal(
    new Label(point(0, 0), 'One\nThree\nTwo', ink(20), new Even()).bounds().width(),
    50,
  );
});

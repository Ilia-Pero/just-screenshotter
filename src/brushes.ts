import { Arrow } from './arrow.ts';
import { Caption } from './caption.ts';
import { Frame } from './frame.ts';
import { ICONS } from './icons.ts';
import { Segment } from './segment.ts';
import { Stroke } from './stroke.ts';
import type { Tool } from './tool.ts';
import { Brush } from './tool.ts';
import type { Words } from './words.ts';

/** A drawing tool as the panel shows it. */
export interface Choice {
  readonly title: string;
  readonly icon: string;
  readonly tool: Tool;
}

/** Drawing tools. unit is the number of screenshot pixels in one pixel of the window. */
export function brushes(words: Words, unit: number): ReadonlyArray<Choice> {
  const thin = 3 * unit;
  return [
    {
      title: words.pencil,
      icon: ICONS.pencil,
      tool: new Brush(thin, 1, 'crosshair', (at, ink) => new Stroke([at], ink)),
    },
    {
      title: words.line,
      icon: ICONS.line,
      tool: new Brush(thin, 1, 'crosshair', (at, ink) => new Segment(at, at, ink)),
    },
    {
      title: words.arrow,
      icon: ICONS.arrow,
      tool: new Brush(thin, 1, 'crosshair', (at, ink) => new Arrow(at, at, ink)),
    },
    {
      title: words.frame,
      icon: ICONS.frame,
      tool: new Brush(thin, 1, 'crosshair', (at, ink) => new Frame(at, at, ink)),
    },
    {
      title: words.marker,
      icon: ICONS.marker,
      tool: new Brush(16 * unit, 0.4, 'crosshair', (at, ink) => new Stroke([at], ink)),
    },
    {
      title: words.text,
      icon: ICONS.text,
      tool: new Brush(1, 1, 'text', (at, ink) => new Caption(at, ink, 18 * unit)),
    },
  ];
}

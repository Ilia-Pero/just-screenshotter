import { Arrow } from './arrow.ts';
import { Caption } from './caption.ts';
import { Frame } from './frame.ts';
import { ICONS } from './icons.ts';
import type { Ruler } from './ruler.ts';
import { Segment } from './segment.ts';
import { Stroke } from './stroke.ts';
import type { Tool } from './tool.ts';
import { Brush, Quill } from './tool.ts';
import type { Words } from './words.ts';

/** A drawing tool as the panel shows it. */
export interface Choice {
  readonly title: keyof Words;
  readonly icon: string;
  readonly tool: Tool;
}

/** Drawing tools. unit is the number of screenshot pixels in one pixel of the window. */
export function brushes(unit: number, ruler: Ruler): ReadonlyArray<Choice> {
  const thin = 3 * unit;
  return [
    {
      title: 'pencil',
      icon: ICONS.pencil,
      tool: new Brush(thin, 1, (at, ink) => new Stroke([at], ink)),
    },
    {
      title: 'line',
      icon: ICONS.line,
      tool: new Brush(thin, 1, (at, ink) => new Segment(at, at, ink)),
    },
    {
      title: 'arrow',
      icon: ICONS.arrow,
      tool: new Brush(thin, 1, (at, ink) => new Arrow(at, at, ink)),
    },
    {
      title: 'frame',
      icon: ICONS.frame,
      tool: new Brush(thin, 1, (at, ink) => new Frame(at, at, ink)),
    },
    {
      title: 'marker',
      icon: ICONS.marker,
      tool: new Brush(16 * unit, 0.4, (at, ink) => new Stroke([at], ink)),
    },
    {
      title: 'text',
      icon: ICONS.text,
      tool: new Quill(new Brush(18 * unit, 1, (at, ink) => new Caption(at, ink, ruler))),
    },
  ];
}

import type { Words } from './words.ts';

/** The colour to draw with until another one is chosen. */
export const RED = '#e5332a';

/** A colour of the palette. */
export interface Color {
  readonly title: string;
  readonly value: string;
}

export function palette(words: Words): ReadonlyArray<Color> {
  return [
    { title: words.red, value: RED },
    { title: words.orange, value: '#f28c28' },
    { title: words.yellow, value: '#f5d000' },
    { title: words.green, value: '#2e9e4f' },
    { title: words.blue, value: '#2f6fdf' },
    { title: words.purple, value: '#8e44ad' },
    { title: words.black, value: '#111111' },
    { title: words.white, value: '#ffffff' },
  ];
}

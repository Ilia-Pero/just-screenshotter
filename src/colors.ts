import type { Words } from './words.ts';

/** The colour to draw with until another one is chosen. */
export const RED = '#e5332a';

/** A colour of the palette. */
export interface Color {
  readonly title: keyof Words;
  readonly value: string;
}

export const PALETTE: ReadonlyArray<Color> = [
  { title: 'red', value: RED },
  { title: 'orange', value: '#f28c28' },
  { title: 'yellow', value: '#f5d000' },
  { title: 'green', value: '#2e9e4f' },
  { title: 'blue', value: '#2f6fdf' },
  { title: 'purple', value: '#8e44ad' },
  { title: 'black', value: '#111111' },
  { title: 'white', value: '#ffffff' },
];

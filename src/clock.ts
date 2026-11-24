function two(value: number): string {
  return value < 10 ? `0${value}` : String(value);
}

/** The time of a film: minutes and seconds, and hours if there are any. */
export function clock(milliseconds: number): string {
  const seconds = Math.max(0, Math.floor(milliseconds / 1000));
  const hours = Math.floor(seconds / 3600);
  const tail = `${two(Math.floor(seconds / 60) % 60)}:${two(seconds % 60)}`;
  return hours > 0 ? `${hours}:${tail}` : tail;
}

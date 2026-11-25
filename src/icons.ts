function icon(body: string): string {
  return (
    '<svg viewBox="0 0 20 20" width="20" height="20" fill="none" stroke="currentColor" ' +
    'stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">' +
    `${body}</svg>`
  );
}

/** Icons of the buttons, as SVG markup. */
export const ICONS = {
  pencil: icon('<path d="M3.5 16.5l.8-3.6 9-9 2.8 2.8-9 9z"/><path d="M11.8 5.4l2.8 2.8"/>'),
  line: icon('<path d="M4 16L16 4"/>'),
  arrow: icon('<path d="M4 16L16 4"/><path d="M9 4h7v7"/>'),
  frame: icon('<rect x="3" y="5" width="14" height="10" rx="1"/>'),
  oval: icon('<ellipse cx="10" cy="10" rx="7.5" ry="5.5"/>'),
  marker: icon('<path d="M5.5 13.5l8-8" stroke-width="5" opacity="0.45"/><path d="M4 17.5h12"/>'),
  text: icon('<path d="M5 5h10"/><path d="M10 5v11"/>'),
  thickness: icon(
    '<path d="M3.5 5h13" stroke-width="1"/><path d="M3.5 9.5h13" stroke-width="2.2"/>' +
      '<path d="M3.5 15h13" stroke-width="4"/>',
  ),
  undo: icon('<path d="M7 4L3 8l4 4"/><path d="M3 8h8.5a4.5 4.5 0 010 9H8"/>'),
  drop: icon(
    '<path d="M3.5 5.5h13"/><path d="M8 5.5V3.5h4v2"/><path d="M5.5 5.5l.7 11h7.6l.7-11"/>' +
      '<path d="M8.5 8.5v5.5"/><path d="M11.5 8.5v5.5"/>',
  ),
  copy: icon(
    '<rect x="7" y="7" width="10" height="10" rx="1.5"/>' +
      '<path d="M13 7V4.5A1.5 1.5 0 0011.5 3h-7A1.5 1.5 0 003 4.5v7A1.5 1.5 0 004.5 13H7"/>',
  ),
  save: icon('<path d="M10 3v10"/><path d="M6 9l4 4 4-4"/><path d="M4 17h12"/>'),
  link: icon(
    '<path d="M8.5 11.5a3 3 0 004.2 0l2.6-2.6a3 3 0 00-4.2-4.2l-1 1"/>' +
      '<path d="M11.5 8.5a3 3 0 00-4.2 0l-2.6 2.6a3 3 0 004.2 4.2l1-1"/>',
  ),
  film: icon('<circle cx="10" cy="10" r="7"/><circle cx="10" cy="10" r="3" fill="currentColor"/>'),
  close: icon('<path d="M5 5l10 10"/><path d="M15 5L5 15"/>'),
};

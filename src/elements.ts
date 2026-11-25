import type { Size } from './layout.ts';
import type { Point } from './point.ts';

export function element<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  name: string,
): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  node.className = name;
  return node;
}

export function button(icon: string): HTMLButtonElement {
  const node = element('button', 'key');
  node.type = 'button';
  node.append(...Array.from(new DOMParser().parseFromString(icon, 'text/html').body.childNodes));
  return node;
}

/** The button that does the act. It does not keep the focus, so that keys go to the editor. */
export function connected(node: HTMLButtonElement, act: () => void): HTMLButtonElement {
  node.addEventListener('click', () => {
    act();
    node.blur();
  });
  return node;
}

/** Gives the element a title that is seen under the mouse and read aloud. */
export function titled(node: HTMLElement, title: string): void {
  node.title = title;
  node.setAttribute('aria-label', title);
}

export function dock(node: HTMLElement, spot: Point): void {
  node.style.left = `${spot.x}px`;
  node.style.top = `${spot.y}px`;
}

export function size(node: HTMLElement): Size {
  return { width: node.offsetWidth, height: node.offsetHeight };
}

import type { Message, Reply, Taken, Wish } from './browser.ts';
import { browser } from './browser.ts';
import { take } from './camera.ts';

/** Pictures that editors have not picked up yet. Each is given away once. */
const kept = new Map<string, Taken>();

function keep(taken: Taken): string {
  const name = crypto.randomUUID();
  kept.set(name, taken);
  return name;
}

async function answered(message: Message): Promise<Reply> {
  if ('take' in message) {
    await take(message.take, keep);
    return { reason: '', taken: [] };
  }
  const taken = kept.get(message.give);
  kept.delete(message.give);
  return { reason: '', taken: taken === undefined ? [] : [taken] };
}

browser.runtime.onMessage.addListener((message, _sender, answer) => {
  answered(message)
    .catch((error: unknown): Reply => ({ reason: String(error), taken: [] }))
    .then(answer)
    .catch(() => {
      // The one who asked has gone.
    });
  return true;
});

// There is no window to complain in, so the icon does.
browser.commands.onCommand.addListener((name) => {
  const wish: Wish = name === 'page' ? 'page' : 'visible';
  take(wish, keep)
    .then(
      () => '',
      () => '!',
    )
    .then((text) => browser.action.setBadgeText({ text }))
    .catch(() => {
      // The icon may be hidden.
    });
});

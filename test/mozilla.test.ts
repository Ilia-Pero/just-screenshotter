import assert from 'node:assert/strict';
import { createHmac } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { Catalog, Extension, Key } from '../tools/mozilla.ts';

const HOME = '/addons/addon/sample@example.org/';
const PLACE = `${HOME}versions/v1.2.3/`;

/** What a catalog says to a question; what is not here it does not know. */
type Answers = Readonly<Record<string, object>>;

/** A catalog that knows nothing of the extension yet. */
const EMPTY: Answers = {
  'POST /addons/upload/': { uuid: 'u1', processed: false, valid: false, validation: {} },
  'GET /addons/upload/u1/': { uuid: 'u1', processed: true, valid: true, validation: {} },
  [`PUT ${HOME}`]: {},
  [`PATCH ${PLACE}`]: {},
  [`GET ${HOME}`]: { url: 'https://catalog.example.org/sample', previews: [] },
  [`PATCH ${HOME}`]: {},
  [`POST ${HOME}previews/`]: {},
};

/** A catalog that has the version, but not the source of it. */
const HALF: Answers = {
  ...EMPTY,
  [`GET ${PLACE}`]: {},
  [`GET ${HOME}`]: { url: 'https://catalog.example.org/sample', previews: [{}] },
};

/** A catalog that has found a mistake. */
const STRICT: Answers = {
  ...EMPTY,
  'GET /addons/upload/u1/': { uuid: 'u1', processed: true, valid: false, validation: 'no icon' },
};

function file(name: string): File {
  return new File(['1'], name);
}

function read(place: string) {
  return JSON.parse(readFileSync(new URL(place, import.meta.url), 'utf8'));
}

/** Sends a sample to a catalog with such answers; the place it is seen at, and the questions. */
async function sent(answers: Answers): Promise<{ place: string; questions: string[] }> {
  const questions: string[] = [];
  const extension = new Extension(
    new Catalog(
      '/',
      () => 'pass',
      (place, letter) => {
        const name = `${letter.method} ${place}`;
        questions.push(name);
        return Promise.resolve(
          Response.json(answers[name] ?? {}, { status: name in answers ? 200 : 404 }),
        );
      },
    ),
    { id: 'sample@example.org', license: 'MIT', notes: '', about: {} },
    () => Promise.resolve(),
  );
  const place = await extension.send({
    version: '1.2.3',
    news: '',
    pack: file('pack.zip'),
    source: file('source.zip'),
    logo: file('logo.png'),
    screen: file('screen.png'),
  });
  return { place, questions };
}

test('a pass is signed with the secret', () => {
  const [head, claim, sign] = new Key('user:1:2', 'secret').pass(1000, 'mark').split('.');
  assert.equal(sign, createHmac('sha256', 'secret').update(`${head}.${claim}`).digest('base64url'));
});

test('a pass names the key and is good for two minutes', () => {
  const claim = new Key('user:1:2', 'secret').pass(1000, 'mark').split('.')[1] ?? '';
  assert.deepEqual(JSON.parse(Buffer.from(claim, 'base64url').toString()), {
    iss: 'user:1:2',
    jti: 'mark',
    iat: 1000,
    exp: 1120,
  });
});

test('a new extension is sent with its source and pictures', async () => {
  assert.deepEqual((await sent(EMPTY)).questions, [
    `GET ${PLACE}`,
    'POST /addons/upload/',
    'GET /addons/upload/u1/',
    `PUT ${HOME}`,
    `PATCH ${PLACE}`,
    `GET ${HOME}`,
    `PATCH ${HOME}`,
    `POST ${HOME}previews/`,
  ]);
});

test('a version that the catalog has is not sent again', async () => {
  assert.deepEqual((await sent(HALF)).questions, [`GET ${PLACE}`, `PATCH ${PLACE}`, `GET ${HOME}`]);
});

test('the place in the catalog is told', async () => {
  assert.equal((await sent(EMPTY)).place, 'https://catalog.example.org/sample');
});

test('a package with a mistake is not published', async () => {
  await assert.rejects(sent(STRICT), /no icon/u);
});

test('what the catalog is told is about this extension', () => {
  assert.equal(
    read('../extension/store/firefox.json').id,
    read('../extension/web/manifest.json').browser_specific_settings.gecko.id,
  );
});

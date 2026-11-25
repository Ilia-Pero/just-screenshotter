// Collects the licenses of the crates the program is built from: the installers carry them.
import { execFileSync } from 'node:child_process';
import { readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { platform } from 'node:process';

/** What cargo says about a crate. */
interface Facts {
  readonly id: string;
  readonly name: string;
  readonly version: string;
  readonly license: string | null;
  readonly authors: readonly string[];
  readonly repository: string | null;
  readonly source: string | null;
  readonly manifest_path: string;
}

/** What cargo says about a program: its crates and what each of them is built from. */
interface Graph {
  readonly packages: readonly Facts[];
  readonly workspace_members: readonly string[];
  readonly resolve: {
    readonly nodes: readonly {
      readonly id: string;
      readonly deps: readonly {
        readonly pkg: string;
        readonly dep_kinds: readonly { readonly kind: string | null }[];
      }[];
    }[];
  };
}

const TARGETS = {
  win32: ['x86_64-pc-windows-msvc'],
  darwin: ['x86_64-apple-darwin', 'aarch64-apple-darwin'],
  other: ['x86_64-unknown-linux-gnu'],
};
const LEGAL = /^(?:licen[cs]e|copying|copyright|notice|unlicense)/iu;

/** A crate of somebody else. */
class Crate {
  private readonly facts: Facts;

  constructor(facts: Facts) {
    this.facts = facts;
  }

  title(): string {
    return `${this.facts.name} ${this.facts.version} (${String(this.facts.license)})`;
  }

  /** The licenses and notices that lie in the crate. */
  texts(): readonly string[] {
    const folder = dirname(this.facts.manifest_path);
    const found = readdirSync(folder, { withFileTypes: true })
      .filter((entry) => entry.isFile() && LEGAL.test(entry.name))
      .map((entry) => readFileSync(join(folder, entry.name), 'utf8').replace(/\r/gu, '').trim());
    return found.length > 0 ? found : [this.origin()];
  }

  private origin(): string {
    return [
      'There is no license text in the crate.',
      `Authors: ${this.facts.authors.join(', ') || 'not named'}`,
      `Source: ${String(this.facts.repository)}`,
    ].join('\n');
  }
}

/** The crates of others that get into the program. */
class Crates {
  private readonly graph: Graph;

  constructor(graph: Graph) {
    this.graph = graph;
  }

  all(): readonly Crate[] {
    const reached = this.reached();
    return this.graph.packages
      .filter((facts) => facts.source !== null && reached.has(facts.id))
      .map((facts) => new Crate(facts));
  }

  /** What the program itself is built from, without what only tests and the build need. */
  private reached(): ReadonlySet<string> {
    const links = new Map(
      this.graph.resolve.nodes.map((node) => [
        node.id,
        node.deps
          .filter((link) => link.dep_kinds.some(({ kind }) => kind === null))
          .map((link) => link.pkg),
      ]),
    );
    const reached = new Set(this.graph.workspace_members);
    for (const id of reached) {
      for (const next of links.get(id) ?? []) {
        reached.add(next);
      }
    }
    return reached;
  }
}

/** The licenses of crates, each text once. */
class Notices {
  private readonly crates: readonly Crate[];

  constructor(crates: readonly Crate[]) {
    this.crates = crates;
  }

  text(): string {
    const owners = new Map<string, string[]>();
    for (const crate of this.crates) {
      for (const text of crate.texts()) {
        owners.set(text, [...(owners.get(text) ?? []), crate.title()]);
      }
    }
    return [
      'The program contains the crates listed here, made by other people, under their licenses.',
      'It also contains the standard library of Rust (MIT OR Apache-2.0), https://github.com/rust-lang/rust',
      ...[...owners].map(
        ([text, titles]) =>
          `${'='.repeat(80)}\n${titles.join('\n')}\n${'-'.repeat(80)}\n${text}`,
      ),
    ].join('\n\n');
  }
}

const targets = platform === 'win32' || platform === 'darwin' ? TARGETS[platform] : TARGETS.other;
const graph: Graph = JSON.parse(
  execFileSync(
    'cargo',
    [
      'metadata',
      '--format-version=1',
      '--locked',
      '--manifest-path=src-tauri/Cargo.toml',
      ...targets.map((target) => `--filter-platform=${target}`),
    ],
    { encoding: 'utf8', maxBuffer: 2 ** 28 },
  ),
);
writeFileSync('src-tauri/licenses.txt', `${new Notices(new Crates(graph).all()).text()}\n`);

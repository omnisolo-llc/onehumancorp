import { readdir, readFile } from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';

async function sourceFiles(root) {
  const entries = await readdir(root, { withFileTypes: true });
  const nested = await Promise.all(entries.map(async (entry) => {
    const file = path.join(root, entry.name);
    if (entry.isDirectory()) return sourceFiles(file);
    return /\.(?:tsx?|mjs)$/.test(entry.name) ? [file] : [];
  }));
  return nested.flat();
}

describe('external font policy', () => {
  it('does not ship browser code that imports remote font assets', async () => {
    const appRoot = path.resolve(import.meta.dirname, '../src/app');
    const offenders = [];

    for (const file of await sourceFiles(appRoot)) {
      const text = await readFile(file, 'utf8');
      if (/fonts\.(?:googleapis|gstatic)\.com/i.test(text)) {
        offenders.push(path.relative(appRoot, file));
      }
    }

    expect(offenders).toEqual([]);
  });
});

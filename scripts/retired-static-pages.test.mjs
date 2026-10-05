import test from 'node:test';
import assert from 'node:assert/strict';
import { access, readdir, readFile } from 'node:fs/promises';

const publicRoot = new URL('../src/ui/next/public/', import.meta.url);
const retired = ['integrations.html', 'ui/integrations.html', 'api-docs.html', 'ui/api-docs.html', 'api/ui/api-docs.html', 'api/v1/ui/api-docs.html', 'trial-extension.html', 'ui/trial-extension.html', 'changelog.html', 'ui/changelog.html', 'api/ui/changelog.html', 'api/v1/ui/changelog.html', 'unified-feed.html', 'ui/unified-feed.html'];

for (const path of retired) {
  test(`web release no longer ships obsolete ${path}`, async () => {
    await assert.rejects(access(new URL(path, publicRoot)), { code: 'ENOENT' });
  });
}

test('shipped navigation does not link to retired static implementations', async () => {
  const stale = [];
  for (const path of await readdir(publicRoot, { recursive: true })) {
    if (!/\.(html|mjs|js)$/.test(path) || retired.includes(path)) continue;
    const source = await readFile(new URL(path, publicRoot), 'utf8');
    if (/["'`](?:\/(?:ui\/|api\/ui\/|api\/v1\/ui\/)?)?(?:api-docs|integrations|trial-extension|changelog|unified-feed)\.html(?:[?#][^"'`]*)?["'`]/.test(source)) stale.push(path);
  }
  assert.deepEqual(stale, [], 'Maintained shipped links must use canonical routes; old URLs remain only as compatibility entry points');
});

test('retirement retains shared helpers and the Tauri bootstrap contract', async () => {
  for (const path of ['help-widget.mjs', 'ui/help-widget.mjs', 'api/v1/ui/help-widget.mjs']) await access(new URL(path, publicRoot));
  await access(new URL('../src/ui/tauri/bootstrap/index.html', import.meta.url));
});

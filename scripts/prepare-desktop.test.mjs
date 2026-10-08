import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';

for (const prepared of ['0', '1']) {
  test(`desktop packaging rejects an unpinned Node before touching resources (prepared=${prepared})`, () => {
    // Keep the version mutation in a child: other tests retain the real runtime.
    const child = spawnSync(process.execPath, ['--input-type=module', '-e', `
      import assert from 'node:assert/strict';
      import { prepareDesktop } from ${JSON.stringify(new URL('./prepare-desktop.mjs', import.meta.url).href)};
      Object.defineProperty(process.versions, 'node', { value: '0.0.0' });
      await assert.rejects(prepareDesktop(), /Desktop packaging requires Node .*found 0\\.0\\.0/);
    `], {
      env: { ...process.env, OMNISOLO_DESKTOP_RESOURCES_PREPARED: prepared,
        OMNISOLO_PREBUILT_WEB: 'target/validation/nonexistent-pin-test-artifact' },
      encoding: 'utf8', timeout: 10_000,
    });
    assert.equal(child.error, undefined);
    assert.equal(child.status, 0, child.stderr);
  });
}

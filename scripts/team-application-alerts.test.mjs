import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';
import { transformSync } from 'esbuild';
import { JSDOM } from './test-support/offline-dom.mjs';

const source = await readFile('src/e2e/support/application_alerts.ts', 'utf8');
const { code } = transformSync(source, { loader: 'ts', format: 'esm', target: 'es2022' });
const { applicationAlertTexts } = await import(`data:text/javascript;base64,${Buffer.from(code).toString('base64')}`);

for (const [name, hostName, id, text, expected] of [
  ['empty Next route announcement', 'next-route-announcer', '__next-route-announcer__', '', []],
  ['populated Next route announcement', 'next-route-announcer', '__next-route-announcer__', 'Team', ['Team']],
  ['ordinary inline error', null, '', 'Unable to load approvals', ['Unable to load approvals']],
  ['body portal error', null, 'portal-error', 'Decision failed', ['Decision failed']],
  ['empty real application alert', null, '', '', ['']],
  ['lookalike ID in light DOM', null, '__next-route-announcer__', 'Real error', ['Real error']],
  ['same ID in another shadow host', 'application-errors', '__next-route-announcer__', 'Real error', ['Real error']],
  ['different alert in announcer host', 'next-route-announcer', 'application-error', 'Real error', ['Real error']],
]) {
  test(`global application alert check: ${name}`, () => {
    const dom = new JSDOM('<body></body>', { runScripts: 'outside-only' });
    try {
      const { document } = dom.window;
      const alert = document.createElement('div');
      alert.setAttribute('role', 'alert'); alert.id = id; alert.textContent = text;
      if (hostName) {
        const host = document.createElement(hostName);
        document.body.append(host); host.attachShadow({ mode: 'open' }).append(alert);
      } else document.body.append(alert);
      // Playwright serializes evaluateAll callbacks into the page. Execute the
      // same callback in a separate DOM realm without imported closures.
      const collect = dom.window.eval(`(${applicationAlertTexts.toString()})`);
      assert.deepEqual(Array.from(collect([alert])), expected);
      const real = document.createElement('div');
      real.setAttribute('role', 'alert'); real.textContent = 'Global application failure';
      document.body.append(real);
      assert.deepEqual(Array.from(collect([alert, real])), [...expected, 'Global application failure']);
    } finally { dom.window.close(); }
  });
}

test('Team still asserts zero application alerts globally after the authenticated read', async () => {
  const team = await readFile('src/e2e/support/team_fixture.ts', 'utf8');
  assert.match(team, /import \{ applicationAlertTexts \} from '\.\/application_alerts'/);
  assert.match(team, /expect\.poll\(\(\) => page\.getByRole\('alert'\)\.evaluateAll\(applicationAlertTexts\)\)\.toEqual\(\[\]\)/);
});

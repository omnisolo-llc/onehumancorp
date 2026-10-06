import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const source = path => readFile(new URL(`../${path}`, import.meta.url), 'utf8');

test('dashboard navigation expects the canonical Help page and real rendered content', async () => {
  const text = await source('src/e2e/help.spec.ts');
  assert.match(text, /toHaveAttribute\("href", "\/help"\)/);
  assert.match(text, /toHaveURL\(url => url.pathname === "\/help"\)/);
});

test('Tauri help exercises the mounted widget, real videos and chat rather than obsolete IDs or fallback-only copy', async () => {
  const text = await source('src/e2e/tauri_help.spec.ts');
  assert.doesNotMatch(text, /#ohc-floating-help-(?:btn|widget|close)|#video-list|dispatchEvent\('click'\)|I am your AI Help Agent!/);
  for (const contract of [/Close Help Widget/, /\/api\/v1\/videos/, /videos\[0\]\.title/, /videos\[0\]\.video_url/, /\/api\/v1\/chat/, /postDataJSON\(\)/, /reply\.reply/, /reply\.link\.url/]) assert.match(text, contract);
});

test('Tauri contextual help uses a mounted tooltip and actual server text', async () => {
  const text = await source('src/e2e/tauri_help.spec.ts');
  assert.doesNotMatch(text, /#generate-link-btn/);
  for (const contract of [/\/api\/v1\/tooltips/, /getByRole\('tooltip'\)/, /toHaveText\(expectedText\)/, /press\('Escape'\)/]) assert.match(text, contract);
});

test('dashboard retirement resolves a standards-valid Location without dropping raw query, owner receipt or history checks', async () => {
  const text = await source('src/ui/next/src/e2e/legacy-dashboard-retirement.spec.ts');
  assert.match(text, /new URL\(response.headers\(\).location, origin\)\.href/);
  for (const contract of [/\['GET', 'HEAD'\]/, /response.status\(\)\)\.toBe\(307\)/, /toBe\(origin \+ '\/dashboard' \+ query\)/, /width: 1440/, /width: 390/, /decision_recorded: true/, /SELECT lifecycle_state/, /page\.goBack\(\)/, /page\.goForward\(\)/]) assert.match(text, contract);
  const origin = 'http://127.0.0.1:18789';
  const query = '?filter=first&filter=second&opaque=%e2%9c%93+%20';
  const expected = origin + '/dashboard' + query;
  for (const location of ['/dashboard' + query, expected]) assert.equal(new URL(location, origin).href, expected);
  for (const location of ['https://other.example/dashboard' + query, '/dashboard?filter=second&filter=first&opaque=%e2%9c%93+%20', '/dashboard?filter=first&filter=second&opaque=%E2%9C%93%20%20']) assert.notEqual(new URL(location, origin).href, expected);
});

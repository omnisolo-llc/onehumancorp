import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import ts from 'typescript';

async function browserTest(filename, title) {
  const source = await readFile(new URL(`../src/e2e/${filename}`, import.meta.url), 'utf8');
  const file = ts.createSourceFile(filename, source, ts.ScriptTarget.Latest, true);
  const matches = [];
  const visit = node => {
    if (ts.isCallExpression(node) && node.expression.getText(file) === 'test'
      && ts.isStringLiteral(node.arguments[0]) && node.arguments[0].text === title) {
      matches.push(node.arguments[1].getText(file));
    }
    ts.forEachChild(node, visit);
  };
  visit(file);
  assert.equal(matches.length, 1, `${filename}: retain the browser scenario ${title}`);
  assert.doesNotMatch(matches[0], /force\s*:\s*true|waitForTimeout|test\.skip/);
  return matches[0];
}

for (const [filename, title] of [
  ['documentation_extended.spec.ts', 'Owner/Operator Persona: Can launch interactive walkthrough from Help widget'],
  ['documentation_verification.spec.ts', 'Dashboard Walkthrough triggers'],
  ['walkthrough_tooltips.spec.ts', 'Dashboard walkthrough and help center elements are visible and work'],
]) test(`${filename}: dashboard tour uses the maintained accessible close control`, async () => {
  const source = await browserTest(filename, title);
  assert.doesNotMatch(source, /omnisolo-walkthrough-close/);
  assert.match(source, /getByRole\('button', \{ name: 'Close walkthrough', exact: true \}\)/);
  assert.match(source, /toBeHidden\(\)/);
});

test('dashboard help uses its mounted launcher and retains close, reopen, Escape and route assertions', async () => {
  const source = await browserTest('global_help_widget.spec.ts', 'should be present and functional on dashboard');
  assert.doesNotMatch(source, /#ohc-floating-help-(?:btn|widget|close)/);
  for (const contract of [/Open help chat/, /Close Help Widget/, /press\('Escape'\)/, /toBeFocused\(\)/, /toHaveURL/]) {
    assert.match(source, contract);
  }
});

for (const [filename, title, gesture] of [
  ['documentation_flows.spec.ts', 'Tooltips load and display properly', /dispatchEvent\('touchstart'\)/],
  ['walkthrough_tooltips.spec.ts', 'Tooltips are injected into the page', /\.hover\(\)/],
]) test(`${filename}: tooltip proof uses the real response and mounted rendered content`, async () => {
  const source = await browserTest(filename, title);
  assert.doesNotMatch(source, /OMNISOLO_TOOLTIPS|#dashboard-walkthrough-btn/);
  for (const contract of [/waitForResponse/, /\/api\/v1\/tooltips/, /status\(\)\)\.toBe\(200\)/, /toHaveAttribute\('data-tooltip', expectedText\)/, /getByRole\('tooltip'\)/, /toHaveText\(expectedText\)/, gesture]) {
    assert.match(source, contract);
  }
});

test('Help video proof waits for real search completion and uses actionable media controls', async () => {
  const source = await browserTest('documentation_features.spec.ts', 'should load Help Center, find videos, and click video to play');
  assert.doesNotMatch(source, /\.evaluate\(|dispatchEvent\(/);
  for (const contract of [/waitForEvent\('requestfinished'/, /url\.origin === origin/, /request\.response\(\)/, /\/api\/v1\/help\/search/, /searchParams\.get\('q'\)/,
    /\/api\/v1\/videos/, /Play video:/, /Close video/, /\.click\(\)/,
    /toHaveAttribute\('src', selectedVideo\.video_url\)/, /toHaveAttribute\('controls'/]) assert.match(source, contract);
});

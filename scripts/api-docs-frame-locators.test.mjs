import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import ts from 'typescript';

function unownedReferenceLocators(source, filename = 'example.spec.ts') {
  const file = ts.createSourceFile(filename, source, ts.ScriptTarget.Latest, true);
  const declarations = new Map();
  const visitDeclarations = node => {
    if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.initializer) {
      const entries = declarations.get(node.name.text) || [];
      entries.push(node.initializer);
      declarations.set(node.name.text, entries);
    }
    ts.forEachChild(node, visitDeclarations);
  };
  visitDeclarations(file);
  const ownsFrame = (expression, seen = new Set()) => {
    if (ts.isIdentifier(expression)) {
      if (seen.has(expression.text)) return false;
      const definitions = declarations.get(expression.text) || [];
      return definitions.length > 0 && definitions.every(value => ownsFrame(value, new Set([...seen, expression.text])));
    }
    if (!ts.isCallExpression(expression) || !ts.isPropertyAccessExpression(expression.expression)) return false;
    if (expression.expression.name.text === 'frameLocator') {
      return expression.arguments.some(argument => argument.getText(file).includes('data-ohc-api-docs-viewer'));
    }
    return ownsFrame(expression.expression.expression, seen);
  };
  const missed = [];
  const visit = node => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression)
      && ['locator', 'getByText', 'getByRole'].includes(node.expression.name.text)
      && node.arguments.some(argument => /OmniSolo Advanced API Reference|API Documentation.*Advanced Users|\.swagger-ui|\.info\s+\.title/.test(argument.getText(file)))
      && !ownsFrame(node.expression.expression)) {
      missed.push(`${filename}:${file.getLineAndCharacterOfPosition(node.getStart(file)).line + 1}: ${node.getText(file)}`);
    }
    ts.forEachChild(node, visit);
  };
  visit(file);
  return missed;
}

for (const selector of [
  "page.getByText('OmniSolo Advanced API Reference')",
  "page.locator('text=OmniSolo Advanced API Reference')",
  "page.getByRole('heading', { name: /^API Documentation \\(for Advanced Users\\)/ })",
  "page.locator('.swagger-ui .info .title')",
]) test(`rejects a parent-owned API reference locator: ${selector}`, () => {
  assert.equal(unownedReferenceLocators(`await expect(${selector}.first()).toBeVisible();`).length, 1);
});

test('accepts frame-owned chains and aliases while preserving parent warning and alias navigation', () => {
  const source = `
    await page.goto('/api/v1/ui/api-docs.html');
    await expect(page.getByText('Advanced:')).toBeVisible();
    const viewer = page.frameLocator('iframe[data-ohc-api-docs-viewer]');
    const swagger = viewer.locator('.swagger-ui');
    await expect(viewer.getByText('OmniSolo Advanced API Reference')).toBeVisible();
    await expect(swagger.locator('.info .title')).toBeVisible();
    await expect(page.frameLocator('iframe[data-ohc-api-docs-viewer]').getByText('OmniSolo Advanced API Reference')).toBeVisible();
  `;
  assert.deepEqual(unownedReferenceLocators(source), []);
});

function trackedBrowserFiles() {
  // Native browser discovery owns all of src, not a fixed set of e2e folders.
  // The tracked inventory also avoids installed dependencies in nested packages.
  return execFileSync('git', ['ls-files', '-z', '--', 'src/**/*.spec.ts'], {
    cwd: fileURLToPath(new URL('..', import.meta.url)), encoding: 'utf8',
  }).split('\0').filter(Boolean);
}

test('tracked browser inventory includes the separate Next e2e root', () => {
  assert.ok(trackedBrowserFiles().includes('src/ui/next/e2e/agents.spec.ts'));
});

test('every tracked browser API reference consumer uses its owning frame', async () => {
  const files = trackedBrowserFiles();
  assert.ok(files.length > 0, 'Scan the maintained browser suites, not an empty inventory');
  const missed = [];
  for (const filename of files) {
    missed.push(...unownedReferenceLocators(await readFile(new URL(`../${filename}`, import.meta.url), 'utf8'), filename));
  }
  assert.deepEqual(missed, [], 'Swagger titles, descriptions and containers belong to the same-origin viewer');
});

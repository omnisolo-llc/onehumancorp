import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

// Keep the shell override aligned with the shared card/panel design token.
// The real-stack smoke suite separately verifies the browser's computed style.
test('shell panel override preserves the sixteen-pixel card radius', () => {
  const css = readFileSync(new URL('../src/ui/next/src/app/globals.css', import.meta.url), 'utf8');
  const shellOverride = css.match(/\.app-shell \.app-card,\s*\.app-shell \.app-panel,\s*\.app-shell \.glassmorphism,\s*\.app-shell \.glass-card\s*\{([^}]+)\}/);
  assert.ok(shellOverride, 'the explicit shared shell card rule must exist');
  assert.match(shellOverride[1], /border-radius:\s*16px\s*!important/);
  assert.doesNotMatch(shellOverride[1], /border-radius:\s*8px/);
});

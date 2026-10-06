import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const source = path => readFileSync(new URL(`../${path}`, import.meta.url), 'utf8');

test('cloud bridge exercises the maintained dashboard and a persisted owned invitation', () => {
  const text = source('src/e2e/growth-cloud-bridge.spec.ts');
  assert.match(text, /createGrowthOwner\(/);
  assert.match(text, /dashboard-viral-invite-widget/);
  assert.match(text, /createRecordedInvitation\(/);
  assert.match(text, /navigator\.clipboard\.readText\(/);
  assert.match(text, /FROM team_invites WHERE tenant_id=\$1/);
  assert.match(text, /page\.reload\(/);
  assert.doesNotMatch(text, /api\/v1\/ui\/dashboard\.html|#generate-link-btn|#share-twitter-btn/);
});

test('manual inventory asserts owned committed adjustments and their persisted readbacks', () => {
  const text = source('src/e2e/tests/centralized_inventory.spec.ts');
  for (const evidence of ['createGrowthOwner(', 'inventory_adjustment_receipts', "status: 'acknowledged'", 'expected_version', 'previous_version', 'adjustment_id', 'page.reload(']) {
    assert.ok(text.includes(evidence), evidence);
  }
  assert.doesNotMatch(text, /\.catch\(|waitForTimeout|optimistic|\.first\(/);
  assert.match(text, /Recorded inventory for your business/);
});

test('missing Terminal connection keeps its exact unavailable receipt and no-charge assertions', () => {
  const text = source('src/ui/next/e2e/terminal_pos.spec.ts');
  assert.match(text, /expect\(token\.status\(\)\)\.toBe\(503\)/);
  assert.match(text, /A verified tenant payment connection is required\./);
  assert.match(text, /status: 'rejected'/);
  assert.match(text, /success: false/);
  assert.match(text, /Charge \\\$/);
  assert.match(text, /Connect.*toHaveCount\(0\)/);
});

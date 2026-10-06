import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
const source = readFileSync(new URL('../src/ui/next/src/e2e/draft_quote_feed.spec.ts', import.meta.url), 'utf8');

test('quote feed browser setup uses the real owner, quote and feed fixtures', () => {
  for (const helper of ['createGrowthOwner', 'createOwnerQuoteFromRequest', 'seedFeedItem', 'loginAs']) {
    assert.match(source, new RegExp(`\\b${helper}\\(`), helper);
  }
  assert.doesNotMatch(source, /page\.route\(|route\.fulfill\(|test@example\.com|password123|x-tenant-id|x-user-id|mockQuoteId/);
  assert.doesNotMatch(source, /request\.post\([^\n]*agent-feed/,
    'feed POST invokes a model; it is not an arbitrary proposed_action fixture endpoint');
});

test('quote approval asserts exact durable receipts and distinguishes execution', () => {
  for (const evidence of ['decision_recorded', 'agent_feed_decisions', 'dispatch', 'job_id',
    'ohc_job_queue', 'lifecycle_state', 'Approval recorded. Execution or delivery is not verified by this decision.']) {
    assert.ok(source.includes(evidence), evidence);
  }
  assert.match(source, /review-quote-draft/);
  assert.match(source, /approve-quote-draft/);
  assert.match(source, /toEqual\(\[\]\)/, 'review cancellation must leave writes unchanged');
  assert.match(source, /page\.reload\(/, 'persisted decision must survive reload');
});

import test from 'node:test';
import assert from 'node:assert/strict';
import { withOwnedBrowserContexts } from './playwright/owned-contexts.mjs';

function browserWith(contexts) {
  let index = 0;
  return { newContext: async () => {
    const result = contexts[index++];
    if (result instanceof Error) throw result;
    return result;
  } };
}

test('releases the first owned context when creating the second fails', async () => {
  const failure = new Error('Second context failed'); const closed = [];
  await assert.rejects(withOwnedBrowserContexts(browserWith([
    { close: async () => closed.push('first') }, failure,
  ]), {}, async () => assert.fail('setup did not finish')), error => error === failure);
  assert.deepEqual(closed, ['first']);
});

test('starts both disposals and waits for both even if the first close fails', async () => {
  const failure = new Error('First context closed'); const closed = [];
  let release;
  const finish = new Promise(resolve => { release = resolve; });
  const result = withOwnedBrowserContexts(browserWith([
    { close: async () => { closed.push('first'); throw failure; } },
    { close: async () => { closed.push('second'); await finish; closed.push('second finished'); } },
  ]), {}, async () => 42);
  const check = result.then(() => null, error => error);
  await new Promise(resolve => setImmediate(resolve));
  const started = [...closed]; release();
  const error = await check;
  assert.deepEqual(started, ['first', 'second']);
  assert.ok(error instanceof AggregateError);
  assert.ok(error.errors.includes(failure));
  assert.deepEqual(closed, ['first', 'second', 'second finished']);
});

test('preserves the original action error alongside every cleanup failure', async () => {
  const primary = new Error('Catalog click timed out');
  const first = new Error('First context already closed'), second = new Error('Second context already closed');
  await assert.rejects(withOwnedBrowserContexts(browserWith([
    { close: async () => { throw first; } }, { close: async () => { throw second; } },
  ]), {}, async () => { throw primary; }), error => {
    assert.ok(error instanceof AggregateError); assert.equal(error.cause, primary);
    assert.deepEqual(error.errors, [primary, first, second]);
    assert.match(error.message, /Catalog click timed out/); return true;
  });
});

test('returns the action result only after disposing all contexts', async () => {
  const closed = [];
  const result = await withOwnedBrowserContexts(browserWith([
    { close: async () => closed.push('first') }, { close: async () => closed.push('second') },
  ]), {}, async contexts => { assert.equal(contexts.length, 2); return 42; });
  assert.equal(result, 42); assert.deepEqual(closed, ['first', 'second']);
});

test('rethrows the identical primary error when cleanup succeeds', async () => {
  const primary = new Error('Catalog click timed out');
  await assert.rejects(withOwnedBrowserContexts(browserWith([
    { close: async () => {} }, { close: async () => {} },
  ]), {}, async () => { throw primary; }), error => error === primary);
});

import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';
import vm from 'node:vm';
import { expect } from '@playwright/test';
import ts from 'typescript';

// Execute the real E2E persistence read and its assertions. Node gates do not
// install a browser; the controlled fetch below supplies actual Response objects.
const filename = new URL('../src/ui/next/e2e/onboarding.spec.ts', import.meta.url);
const source = ts.createSourceFile(filename.pathname, await readFile(filename, 'utf8'), ts.ScriptTarget.Latest, true);
let statements;
function findRead(node) {
  if (ts.isBlock(node)) {
    const index = node.statements.findIndex(statement => ts.isVariableStatement(statement)
      && statement.declarationList.declarations.some(declaration => declaration.name.getText(source) === 'state'));
    if (index !== -1) {
      assert.equal(statements, undefined, 'the final persisted-state read must be unambiguous');
      statements = node.statements.slice(index, index + 3);
    }
  }
  ts.forEachChild(node, findRead);
}
findRead(source);
assert.equal(statements?.length, 3, 'include the real read, HTTP status assertion and durable-state assertion');
const block = statements.map(statement => statement.getText(source)).join('\n');
const javascript = ts.transpileModule(block, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
const verifyState = vm.runInNewContext(`(async (page, expect, identity, prepared) => { ${javascript} })`);
const identity = { tenantId: 'owned-tenant', userId: 'owned-user' };
const prepared = { preparation_id: 'owned-preparation' };
const preparation = { status: 'launched', preparation_id: prepared.preparation_id, organization_id: identity.tenantId, user_id: identity.userId };

function browserRead(fetchResponse) {
  let evaluations = 0;
  const requests = [];
  const page = {
    evaluate: callback => {
      evaluations++;
      return vm.runInNewContext(`(${callback.toString()})()`, {
        fetch: async (url, options) => {
          requests.push({ url, options });
          return fetchResponse();
        },
      });
    },
  };
  return {
    run: () => verifyState(page, expect, identity, prepared),
    assertSingleRead: () => {
      assert.equal(evaluations, 1);
      assert.equal(requests.length, 1);
      assert.equal(requests[0].url, '/api/v1/onboarding/state');
      assert.equal(requests[0].options?.cache, 'no-store');
      assert.equal(requests[0].options?.method ?? 'GET', 'GET');
    },
  };
}

test('onboarding verifies persisted launch once through the current browser session', async () => {
  const read = browserRead(() => Response.json({ preparation }));
  await read.run();
  read.assertSingleRead();
});

for (const [name, body, status] of [
  ['non-200 response', { preparation }, 503],
  ['unlaunched preparation', { preparation: { ...preparation, status: 'prepared' } }, 200],
  ['different preparation', { preparation: { ...preparation, preparation_id: 'other' } }, 200],
  ['foreign tenant', { preparation: { ...preparation, organization_id: 'foreign' } }, 200],
  ['foreign owner', { preparation: { ...preparation, user_id: 'foreign' } }, 200],
]) {
  test(`onboarding persistence assertion rejects ${name}`, async () => {
    const read = browserRead(() => Response.json(body, { status }));
    await assert.rejects(read.run(), error => Boolean(error.matcherResult));
    read.assertSingleRead();
  });
}

test('onboarding persistence read propagates a network failure without retry', async () => {
  const failure = new TypeError('controlled connection reset');
  const read = browserRead(() => { throw failure; });
  await assert.rejects(read.run(), error => error === failure);
  read.assertSingleRead();
});

test('onboarding persistence read rejects malformed JSON without retry', async () => {
  const read = browserRead(() => new Response('not json', { status: 200 }));
  await assert.rejects(read.run(), SyntaxError);
  read.assertSingleRead();
});

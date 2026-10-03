import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import ts from 'typescript';

const text = await readFile('src/e2e/comprehensive_ui_contract.spec.ts', 'utf8');
const source = ts.createSourceFile('audit.ts', text, ts.ScriptTarget.Latest, true);
const declaration = source.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === 'normalizeInternalHref');
assert.ok(declaration);
const compiled = ts.transpileModule(declaration.getText(source), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
const normalize = new Function(`${compiled};return normalizeInternalHref;`)();

test('absolute links to the actual app origin remain checked internal destinations', () => {
  assert.equal(normalize('http://127.0.0.1:44041/onboarding?ref=owned', 'http://127.0.0.1:44041/wrapped'), '/onboarding?ref=owned');
  assert.equal(normalize('https://business.example/orders/owned', 'https://business.example/dashboard'), '/orders/owned');
  assert.equal(normalize('/help', 'https://business.example/dashboard'), '/help');
  assert.equal(normalize('//business.example/help', 'https://business.example/dashboard'), '/help');
});
test('foreign origins, credentials and distinct local ports are never reclassified as app links', () => {
  for (const href of ['http://127.0.0.1:44042/onboarding', 'https://untrusted.example/path', 'https://user:password@business.example/path', 'http://dummy.base/path']) {
    assert.equal(normalize(href, 'https://business.example/wrapped'), null);
  }
  assert.equal(normalize('http://127.0.0.1:44042/onboarding', 'http://127.0.0.1:44041/wrapped'), null);
  assert.equal(normalize('http://business.example/path', 'https://business.example/wrapped'), null);
  assert.equal(normalize('//untrusted.example/path', 'https://business.example/wrapped'), null);
  assert.equal(normalize('http://[::broken]/path', 'https://business.example/wrapped'), null);
});

test('a malformed external link is reported while the actual sweep continues checking later links', async () => {
  const helperNames = new Set(['normalizeInternalHref', 'routeLabel', 'externalHostAllowed', 'isFakeOmniSoloUrl']);
  const helpers = source.statements.filter(node => ts.isFunctionDeclaration(node) && helperNames.has(node.name?.text)
    || ts.isVariableStatement(node) && node.declarationList.declarations.some(declaration => declaration.name.getText(source) === 'allowedExternalHosts'));
  let loop;
  const visit = node => {
    if (ts.isForOfStatement(node) && node.initializer.getText(source) === 'const link' && node.expression.getText(source) === 'hrefs') loop = node;
    ts.forEachChild(node, visit);
  };
  visit(source); assert.ok(loop);
  const script = ts.transpileModule(`${helpers.map(node => node.getText(source)).join('\n')}\nasync function run(hrefs) {const failures=[];const route='/audited-route';const page={url:()=> 'https://business.example/current'};${loop.getText(source)};return failures;}`, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  const run = new Function(`${script};return run;`)();
  const failures = await run([
    { href: 'http://[broken', index: 0, text: 'Malformed link' },
    { href: 'https://untrusted.example/path', index: 1, text: 'Later foreign link' },
  ]);
  assert.equal(failures.length, 2);
  assert.match(failures[0], /audited-route.*Malformed link.*invalid URL/);
  assert.match(failures[1], /Later foreign link.*unexpected external host/);
});

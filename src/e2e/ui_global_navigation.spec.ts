import { test, expect } from '@playwright/test';
import { createServer } from 'node:http';
import type { AddressInfo } from 'node:net';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import ts from 'typescript';
import type { Page } from '@playwright/test';
import { createAuditNavigation } from './support/ui_audit_navigation';

// Extract the actual navigation expression used by each global audit. This
// catches a caller bypassing the already-tested settled navigation helper.
const file = path.resolve(process.cwd(), 'src/e2e/comprehensive_ui_contract.spec.ts');
const source = ts.createSourceFile(file, readFileSync(file, 'utf8'), ts.ScriptTarget.Latest, true);
const names = [
  'every app page loads without visible crash output',
  'visible internal links resolve to real pages',
  'visible external and protocol links use expected destinations',
  'all visible interactive elements are usable and named',
  'layouts do not overflow or overlap click targets on desktop and mobile',
];
function expressionFor(name: string): string {
  let expression: string | undefined;
  const visit = (node: ts.Node) => {
    if (ts.isCallExpression(node) && node.expression.getText(source) === 'test'
      && ts.isStringLiteral(node.arguments[0]) && node.arguments[0].text === name) {
      const find = (child: ts.Node) => {
        if (ts.isCallExpression(child) && ['page.goto', 'gotoReady'].includes(child.expression.getText(source))) {
          if (expression) throw new Error('Global audit has multiple navigation entry points');
          expression = child.getText(source);
        }
        ts.forEachChild(child, find);
      };
      find(node.arguments[1]);
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
  if (!expression) throw new Error(`No actual navigation found for ${name}`);
  return expression;
}
for (const name of names) {
  test(`global audit settles declared redirects before reading its next document: ${name}`, async ({ page }) => {
    const server = createServer((request, response) => {
      response.setHeader('content-type', 'text/html; charset=utf-8');
      response.setHeader('cache-control', 'no-store');
      if (request.url === '/share-card') {
        // A streamed/hydrated refresh can be inserted after DOMContentLoaded.
        response.end(`<button id="transient">Transient share shell</button><script>setTimeout(()=>{const meta=document.createElement('meta');meta.httpEquiv='refresh';meta.content='0;url=/onboarding';document.head.append(meta)},600)</script>`);
      } else if (request.url === '/onboarding') response.end('<a href="/share-cards">Saved share cards</a><button id="onboarding">Ready onboarding</button>');
      else if (request.url === '/share-cards') response.end('<a href="/onboarding">Onboarding</a><button id="share-cards">Saved cards</button>');
      else response.writeHead(404).end('Not found');
    });
    await new Promise<void>((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
    const origin = `http://127.0.0.1:${(server.address() as AddressInfo).port}`;
    const navigate = createAuditNavigation(origin, async () => {});
    const actualNavigation = new Function('page', 'route', 'gotoReady', 'process', `return ${expressionFor(name)}`) as (page: Page, route: string, ready: typeof navigate, process: { env: { BASE_URL: string } }) => Promise<unknown>;
    try {
      for (const [route, finalPath, id] of [['/share-card', '/onboarding', 'onboarding'], ['/share-cards', '/share-cards', 'share-cards']]) {
        await actualNavigation(page, route, navigate, { env: { BASE_URL: origin } });
        expect(new URL(page.url()).pathname).toBe(finalPath);
        const documentId = await page.evaluate(() => { const id = crypto.randomUUID(); document.documentElement.dataset.auditDocument = id; return id; });
        const controls = await page.locator('a[href], button').evaluateAll(nodes => nodes.map(node => ({ id: node.id, href: node.getAttribute('href'), text: node.textContent, width: node.getBoundingClientRect().width })));
        expect(controls.some(control => control.id === id && control.width > 0)).toBe(true);
        expect(controls.some(control => control.id === 'transient')).toBe(false);
        expect(await page.evaluate(() => document.documentElement.dataset.auditDocument)).toBe(documentId);
      }
    } finally {
      await page.goto('about:blank').catch(() => undefined);
      server.closeAllConnections();
      await new Promise<void>((resolve, reject) => server.close(error => error ? reject(error) : resolve()));
    }
  });
}

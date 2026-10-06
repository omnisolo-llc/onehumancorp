import { test, expect } from '@playwright/test';
import { createServer } from 'node:http';
import type { AddressInfo } from 'node:net';
import type { Page } from '@playwright/test';
import { createAuditNavigation } from './support/ui_audit_navigation';

async function withRedirectFixture(page: Page, target: '/onboarding' | '/pos/terminal' | '/wrong', run: (origin: string) => Promise<void>, sourceRoute: '/share-card' | '/pos' = '/share-card') {
  const server = createServer((request, response) => {
    if (request.method !== 'GET') { response.writeHead(405).end(); return; }
    response.setHeader('content-type', 'text/html; charset=utf-8');
    response.setHeader('cache-control', 'no-store');
    response.setHeader('x-content-type-options', 'nosniff');
    if (request.url === sourceRoute) {
      if (sourceRoute === '/pos') { response.writeHead(307, { location: target }).end(); return; }
      response.end(`<button id="transient">Transient layout control</button><script>setTimeout(() => location.replace(${JSON.stringify(target)}), 600)</script>`);
    } else if (request.url === '/onboarding' || request.url === '/pos/terminal') {
      response.end('<button id="ready" onclick="document.querySelector(\'output\').textContent=\'Actual final-page effect\'">Ready action</button><output></output>');
    } else if (request.url === '/wrong') response.end('<button>Unexpected destination</button>');
    else response.writeHead(404).end();
  });
  await new Promise<void>((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  const origin = `http://127.0.0.1:${(server.address() as AddressInfo).port}`;
  try { await run(origin); }
  finally {
    await page.goto('about:blank').catch(() => undefined);
    server.closeAllConnections();
    await new Promise<void>((resolve, reject) => server.close(error => error ? reject(error) : resolve()));
  }
}

test('audit waits for the real delayed share-card navigation before inspecting or clicking controls', async ({ page }) => {
  await withRedirectFixture(page, '/onboarding', async origin => {
    const navigate = createAuditNavigation(origin, async () => {});
    const receipt = await navigate(page, '/share-card');
    expect(receipt).toEqual({ requestedUrl: `${origin}/share-card`, finalUrl: `${origin}/onboarding`, redirected: true });
    await expect(page.locator('#transient')).toHaveCount(0);
    await page.getByRole('button', { name: 'Ready action' }).click();
    await expect(page.locator('output')).toHaveText('Actual final-page effect');
  });
});

test('audit rejects an actual unclassified destination without clicking its controls', async ({ page }) => {
  test.setTimeout(15000);
  await withRedirectFixture(page, '/wrong', async origin => {
    const navigate = createAuditNavigation(origin, async () => {});
    await expect(navigate(page, '/share-card')).rejects.toThrow();
    await expect(page.getByRole('button', { name: 'Unexpected destination' })).toBeVisible();
  });
});

test('audit rejects external and recursive declared redirects before any browser navigation', async ({ page }) => {
  let authentications = 0;
  const navigate = createAuditNavigation('http://127.0.0.1:1', async () => { authentications += 1; });
  for (const route of ['https://outside.invalid', '/share-card?url=%2Fshare-card', '/share-card?url=https%3A%2F%2Foutside.invalid']) {
    await expect(navigate(page, route)).rejects.toThrow(/destination|redirect/i);
  }
  expect(authentications).toBe(0);
  expect(page.url()).toBe('about:blank');
});


test('audit follows the real POS server redirect and exercises the final terminal controls', async ({ page }) => {
  await withRedirectFixture(page, '/pos/terminal', async origin => {
    const navigate = createAuditNavigation(origin, async () => {});
    const receipt = await navigate(page, '/pos');
    expect(receipt).toEqual({ requestedUrl: `${origin}/pos`, finalUrl: `${origin}/pos/terminal`, redirected: true });
    await page.getByRole('button', { name: 'Ready action' }).click();
    await expect(page.locator('output')).toHaveText('Actual final-page effect');
  }, '/pos');
});

test('audit refuses an unexpected POS server redirect before clicking controls', async ({ page }) => {
  await withRedirectFixture(page, '/wrong', async origin => {
    const navigate = createAuditNavigation(origin, async () => {});
    await expect(navigate(page, '/pos')).rejects.toThrow(/unclassified destination/);
    await expect(page.getByRole('button', { name: 'Unexpected destination' })).toBeVisible();
  }, '/pos');
});

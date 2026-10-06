import { test, expect } from '../../../../e2e/fixtures';

const aliases = [
  ['/integrations.html', '/integrations'],
  ['/ui/integrations.html', '/integrations'],
  ['/api-docs.html', '/api-docs'],
  ['/ui/api-docs.html', '/api-docs'],
  ['/api/ui/api-docs.html', '/api-docs'],
  ['/api/v1/ui/api-docs.html', '/api-docs'],
] as const;

for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
  test.describe(`retired static pages at ${viewport.width}px`, () => {
    test.use({ viewport });

    for (const [legacy, canonical] of aliases) {
      test(`${legacy} reaches real ${canonical} content`, async ({ page }, testInfo) => {
        const missingAssets: string[] = [];
        page.on('response', (response) => {
          if (response.status() >= 400 && ['script', 'stylesheet'].includes(response.request().resourceType())) missingAssets.push(response.url());
        });
        page.on('requestfailed', (request) => {
          if (['script', 'stylesheet'].includes(request.resourceType())) missingAssets.push(`${request.url()}: ${request.failure()?.errorText}`);
        });
        const apiPath = canonical === '/api-docs' ? '/api/v1/api-docs-spec' : '/api/v1/integrations';
        const apiResponse = page.waitForResponse((response) => new URL(response.url()).pathname === apiPath && response.request().method() === 'GET');
        const redirectResponse = page.waitForResponse((response) => new URL(response.url()).pathname === legacy);
        await page.goto(`${legacy}?audit=compatibility#reference`, { waitUntil: 'domcontentloaded' });
        await expect(page).toHaveURL(new RegExp(`${canonical.replaceAll('/', '\\/')}\\?audit=compatibility#reference$`));
        const redirect = await redirectResponse;
        expect(redirect.status()).toBe(307);
        expect(redirect.headers()['cache-control']).toContain('no-store');
        const response = await apiResponse;
        expect(response.status()).toBe(200);
        const data = await response.json();
        if (canonical === '/api-docs') {
          expect(data.paths).toBeTruthy();
          await expect(page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('.swagger-ui')).toBeVisible();
          const explanation = page.locator('#api-docs-tooltip');
          await explanation.focus();
          const tooltip = page.getByRole('tooltip');
          await expect(tooltip).toHaveText('Direct API access is only for custom integrations.');
          await expect(tooltip).toHaveCount(1);
          await expect(explanation).toHaveAttribute('aria-describedby', 'api-docs-tooltip-description');
          await explanation.press('Escape');
          await expect(tooltip).toBeHidden();
          await expect(explanation).toBeFocused();
        } else {
          expect(Array.isArray(data.integrations)).toBe(true);
          await expect(page.getByRole('heading', { name: 'Verified business connections' })).toBeVisible();
          await expect(page.getByRole('heading', { name: 'Stripe payments' })).toBeVisible();
        }
        expect(missingAssets).toEqual([]);
        await testInfo.attach('canonical-page', { body: await page.screenshot(), contentType: 'image/png' });
      });
    }
  });
}

test.describe('retirement keeps existing access boundaries', () => {
  for (const [legacy] of aliases) {
    test(`anonymous ${legacy} is still protected`, async ({ anonymousPage }) => {
      const response = await anonymousPage.request.get(legacy, { maxRedirects: 0 });
      expect(response.status()).toBe(legacy.startsWith('/api/') ? 401 : 307);
      if (!legacy.startsWith('/api/')) {
        const location = new URL(response.headers().location, response.url());
        expect(location.pathname).toBe('/login');
        expect(location.searchParams.get('next')).toBe(legacy);
      }
      expect(response.headers()['cache-control']).toContain('no-store');
    });
  }

  test('public entry points remain available without opening arbitrary publication documents', async ({ anonymousPage }) => {
    for (const path of ['/login', '/register', '/verify-email', '/healthz']) {
      const response = await anonymousPage.request.get(path, { maxRedirects: 0 });
      expect(response.status(), path).toBe(200);
    }
    const publication = await anonymousPage.request.get('/api/v1/public/sites/00000000-0000-0000-0000-000000000000', { maxRedirects: 0 });
    expect(publication.status()).toBe(404);
    expect(publication.headers()['cache-control']).toContain('no-store');
  });
});

test('shipped dashboard and Help links reach the canonical applications', async ({ page }) => {
  await page.goto('/dashboard.html', { waitUntil: 'domcontentloaded' });
  const integrations = page.locator('a[href="/integrations"]').first();
  await expect(integrations).toBeVisible();
  await integrations.click();
  await expect(page).toHaveURL(/\/integrations$/);
  await expect(page.getByRole('heading', { name: 'Verified business connections' })).toBeVisible();

  await page.goto('/ui/help.html', { waitUntil: 'domcontentloaded' });
  const documentation = page.locator('a[href="/api-docs"]').filter({ hasText: 'API Documentation' }).first();
  await expect(documentation).toBeVisible();
  await documentation.click();
  await expect(page).toHaveURL(/\/api-docs$/);
  await expect(page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('.swagger-ui')).toBeVisible();
});

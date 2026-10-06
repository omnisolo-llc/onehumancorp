import { test, expect } from '../fixtures';

test.describe('API Documentation', () => {
  for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
    test(`legacy API docs render and expand the real specification at ${viewport.width}px`, async ({ page }, testInfo) => {
      await page.setViewportSize(viewport);
      const loaded = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/api-docs-spec' && response.request().method() === 'GET');
      await page.goto('/api/v1/ui/api-docs.html');
      await expect(page).toHaveURL(/\/api-docs$/);

      const response = await loaded;
      expect(response.status()).toBe(200);
      const spec = await response.json();
      expect(spec).toMatchObject({ openapi: '3.0.0', info: { title: 'API Documentation (for Advanced Users)' } });
      expect(spec.paths['/api/v1/help'].get.summary).toBe('Get Help Articles');
      const swagger = page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('.swagger-ui');
      await expect(swagger).toBeVisible();
      await expect(swagger.locator('.info .title')).toContainText(spec.info.title);
      await expect(swagger.locator('.info .description')).toContainText(spec.info.description);

      // Opening the real operation proves the maintained reference is usable,
      // rather than merely finding an empty Swagger container.
      const operation = swagger.locator('.opblock-get').filter({
        has: page.locator('.opblock-summary-path').filter({ hasText: /^\/api\/v1\/help$/ }),
      });
      const summary = operation.locator('button.opblock-summary-control');
      await expect(summary).toContainText(spec.paths['/api/v1/help'].get.summary);
      await expect(summary).toHaveAttribute('aria-expanded', 'false');
      await summary.click();
      await expect(summary).toHaveAttribute('aria-expanded', 'true');
      const description = operation.locator('.opblock-description-wrapper').filter({ hasText: spec.paths['/api/v1/help'].get.description });
      await expect(description).toBeVisible();
      await expect(description).toHaveText(spec.paths['/api/v1/help'].get.description);
      await expect(operation.locator('.responses-table')).toBeVisible();
      await expect(operation.locator('.responses-table')).toContainText(spec.paths['/api/v1/help'].get.responses['200'].description);
      await testInfo.attach('expanded-api-reference', { body: await page.screenshot(), contentType: 'image/png' });
      await summary.click();
      await expect(summary).toHaveAttribute('aria-expanded', 'false');
      await expect(operation.locator('.responses-table')).toBeHidden();
    });
  }
});

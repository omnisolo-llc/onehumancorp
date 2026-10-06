import { test, expect } from './fixtures';

test.describe('API Documentation', () => {
  test('should display interactive Swagger UI layout', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Navigate to API Docs page
    await page.goto('/api-docs');

    // Ensure the advanced warning is visible
    await expect(page.locator('text=Advanced:')).toBeVisible();
    await expect(page.getByText('This section is for developers directly integrating with our APIs.')).toBeVisible();

    // Tooltip hover test
    const tooltipTarget = page.locator('#api-docs-tooltip');
    await tooltipTarget.waitFor({ state: "visible", timeout: 10000 });
    await tooltipTarget.hover();

    const tooltipElement = page.locator('[role="tooltip"]');
    await expect(tooltipElement).toBeVisible();
    await expect(tooltipElement).toContainText('Direct API access is only for custom integrations.');

    // Verify Swagger UI container wrapper is visible
    // Target the specific wrapper classes for verification
    const wrapper = page.locator('.backdrop-blur-\\[30px\\]').first();
    await expect(wrapper).toBeVisible();

    // Check if swagger-ui container renders
    const swaggerUI = page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('.swagger-ui');
    await expect(swaggerUI).toBeVisible();
  });

  test('should not have horizontal scroll issues on mobile viewport', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Set viewport to mobile (375px)
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/api-docs');

    // Wait for the swagger UI to load
    await expect(page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('.swagger-ui')).toBeVisible({ timeout: 15000 });

    // Check if layout allows for horizontal scroll by evaluating the clientWidth vs scrollWidth
    const overflowInfo = await page.evaluate(() => {
      const scrollWidth = document.documentElement.scrollWidth;
      const clientWidth = document.documentElement.clientWidth;
      return { scrollWidth, clientWidth, hasHorizontalScroll: scrollWidth > clientWidth };
    });

    // In a well-behaved mobile design, it shouldn't allow horizontal scroll at the root
    expect(overflowInfo.hasHorizontalScroll).toBe(false);
    const frameOverflow = await page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('html').evaluate(element => element.scrollWidth > element.clientWidth);
    expect(frameOverflow).toBe(false);
  });
  test('keyboard operations and the real renderer survive client navigation and back', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/api-docs');
    for (let visit = 0; visit < 2; visit += 1) {
      const iframe = page.locator('iframe[data-ohc-api-docs-viewer]');
      await expect(iframe).toHaveAttribute('title', 'Interactive API documentation');
      await expect(iframe).toHaveAttribute('aria-busy', 'false', { timeout: 15000 });
      const handle = await iframe.elementHandle();
      const renderer = await handle!.contentFrame();
      expect(renderer).not.toBeNull();
      const operation = page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('.opblock-summary-control').first();
      await expect(operation).toHaveAttribute('aria-expanded', 'false');
      await operation.focus();
      await operation.press('Enter');
      await expect(operation).toHaveAttribute('aria-expanded', 'true');
      await operation.press('Enter');
      await expect(operation).toHaveAttribute('aria-expanded', 'false');
      await page.locator('#help-center-nav-btn').click();
      await expect(page).toHaveURL(/\/help$/);
      await expect(iframe).toHaveCount(0);
      expect(renderer!.isDetached()).toBe(true);
      await handle!.dispose();
      const loaded = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/api-docs-spec' && response.request().method() === 'GET');
      await page.goBack();
      expect((await loaded).status()).toBe(200);
      await expect(page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('.swagger-ui')).toBeVisible();
    }
  });

});

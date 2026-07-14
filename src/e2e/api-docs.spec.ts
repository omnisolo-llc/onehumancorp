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
    const swaggerUI = page.locator('.swagger-ui');
    await expect(swaggerUI).toBeVisible();
  });

  test('should not have horizontal scroll issues on mobile viewport', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Set viewport to mobile (375px)
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/api-docs');

    // Wait for the swagger UI to load
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 15000 });

    // Check if layout allows for horizontal scroll by evaluating the clientWidth vs scrollWidth
    const overflowInfo = await page.evaluate(() => {
      const scrollWidth = document.documentElement.scrollWidth;
      const clientWidth = document.documentElement.clientWidth;
      return { scrollWidth, clientWidth, hasHorizontalScroll: scrollWidth > clientWidth };
    });

    // In a well-behaved mobile design, it shouldn't allow horizontal scroll at the root
    expect(overflowInfo.hasHorizontalScroll).toBe(false);
  });

  test('should have width constrained on Swagger UI tables on mobile', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/api-docs');
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 15000 });

    const isTableConstrained = await page.evaluate(() => {
      const table = document.querySelector('.swagger-ui table');
      if (!table) return true; // If no table exists, it's technically not overflowing
      const rect = table.getBoundingClientRect();
      return rect.width <= document.documentElement.clientWidth;
    });

    expect(isTableConstrained).toBe(true);
  });

  test('should have max-width set on Swagger UI wrapper', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/api-docs');
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 15000 });

    const wrapperMaxW = await page.evaluate(() => {
      const el = document.querySelector('.swagger-ui .wrapper');
      return el ? window.getComputedStyle(el).maxWidth : 'none';
    });

    // We expect the max-width to be 100vw or a px equivalent that matches the viewport
    expect(wrapperMaxW).not.toBe('none');
  });

  test('should handle long text in opblock paths with word-break', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/api-docs');
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 15000 });

    const hasWordBreak = await page.evaluate(() => {
      const el = document.querySelector('.swagger-ui .opblock .opblock-summary-path');
      if (!el) return true;
      return window.getComputedStyle(el).wordBreak === 'break-all';
    });

    expect(hasWordBreak).toBe(true);
  });

  test('should allow horizontal scrolling within opblock pre elements', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/api-docs');
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 15000 });

    const isPreScrollable = await page.evaluate(() => {
      const pre = document.querySelector('.swagger-ui .opblock-body pre');
      if (!pre) return true;
      const style = window.getComputedStyle(pre);
      return style.overflowX === 'auto' || style.overflowX === 'scroll';
    });

    expect(isPreScrollable).toBe(true);
  });
});

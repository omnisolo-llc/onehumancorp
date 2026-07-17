import { test, expect } from './fixtures';

test.describe('API Documentation', () => {
  test('should display interactive Swagger UI layout', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Navigate to API Docs page
    await page.goto('/api-docs.html');

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
    await page.goto('/api-docs.html');

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
});

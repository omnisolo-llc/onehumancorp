import { test, expect } from './fixtures';

test.describe('API Documentation', () => {
  test('should display interactive Swagger UI layout', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Navigate to API Docs page
    await page.goto('/api/ui/api-docs.html');

    // Advanced Settings toggle
    const toggle = page.locator('#advanced-settings-toggle');
    const apiDocsContent = page.locator('#api-docs-content');

    // Check initial state (should be hidden)
    await expect(apiDocsContent).not.toBeVisible();

    // Click toggle
    await toggle.check();

    // Check state after click (should be visible)
    await expect(apiDocsContent).toBeVisible();

    // Ensure the advanced warning is visible
    await expect(page.locator('text=Advanced:')).toBeVisible();
    await expect(page.getByText('This section is for developers directly integrating with our APIs.')).toBeVisible();

    // Tooltip hover test
    const tooltipTarget = page.locator('#api-docs-tooltip');
    await tooltipTarget.hover();

    // The script appends three `.ohc-tooltip` elements for some reason depending on duplicated scripts
    const tooltipElement = page.locator('.ohc-tooltip.visible').first();
    await expect(tooltipElement).toBeVisible();
    // Use an assertion that checks if the fetched dynamic text or the fallback exists
    const textContext = await tooltipElement.textContent();
    expect(
        textContext?.includes('Direct API access is only for custom integrations.') ||
        textContext?.includes('Connect custom tools with your account.')
    ).toBe(true);

    // Verify Swagger UI container wrapper is visible
    // Target the specific wrapper classes for verification
    const wrapper = page.locator('.glassmorphism').first();
    await expect(wrapper).toBeVisible();

    // Check if swagger-ui container renders
    const swaggerUI = page.locator('#swagger-ui');
    await expect(swaggerUI).toBeVisible();
  });

  test('should not have horizontal scroll issues on mobile viewport', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Set viewport to mobile (375px)
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/api/ui/api-docs.html');

    // Advanced Settings toggle for the swagger UI to show
    const toggle = page.locator('#advanced-settings-toggle');
    await toggle.check();

    // Wait for the swagger UI to load
    await expect(page.locator('#swagger-ui')).toBeVisible({ timeout: 15000 });

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

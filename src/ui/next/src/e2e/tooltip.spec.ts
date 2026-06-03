import { test, expect } from '@playwright/test';

test.describe('Tooltips', () => {
  test('should display tooltips on hover', async ({ page }) => {
    // Navigate to the Dashboard page which uses WithTooltip
    await page.goto('/dashboard');

    // Wait for a tooltip element to be present
    // For example, the total sales stat
    const totalSalesLocator = page.locator('div[class*="cursor-help"]').first();
    await expect(totalSalesLocator).toBeVisible();

    // Hover over the element
    await totalSalesLocator.hover();

    // Wait for the tooltip text to appear
    // The tooltip itself is placed in a fixed absolute container
    const tooltipText = page.locator('div.animate-fade-in-up.text-white.text-sm');
    await expect(tooltipText).toBeVisible();

    // Assert there's some text
    const textContent = await tooltipText.textContent();
    expect(textContent?.length).toBeGreaterThan(0);

    // Un-hover to close
    await page.mouse.move(0, 0);
    await expect(tooltipText).not.toBeVisible();
  });
});

import { test, expect } from '@playwright/test';

test.describe('Tooltip functionality', () => {
  test('shows tooltip on hover', async ({ page }) => {
    await page.goto('/pricing');

    // The heading wrapped in WithTooltip
    const trigger = page.locator('h1', { hasText: 'Pricing Plans' });
    await expect(trigger).toBeVisible();

    // Hover over the trigger
    await trigger.hover();

    // Check if the tooltip with default text or fetched text is visible
    const tooltipText = "Select the plan that best fits your business needs.";
    const tooltipDiv = page.locator(`text="${tooltipText}"`);

    await expect(tooltipDiv).toBeVisible();

    // Hover away
    await page.locator('text=Back to Dashboard').hover();

    // Wait for the tooltip to disappear
    await expect(tooltipDiv).toBeHidden();
  });
});

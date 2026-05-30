import { test, expect } from '@playwright/test';

test.describe('Tooltip functionality', () => {
  test('shows tooltip on hover', async ({ page }) => {
    await page.goto('/pricing');

    // The heading wrapped in WithTooltip
    const trigger = page.locator('h1', { hasText: 'Pricing Plans' });
    await expect(trigger).toBeVisible();

    // Hover over the trigger
    await trigger.hover();

    // The tooltip uses a generic string from API or the default text if API fails.
    // Given the component renders text in a div that is conditionally shown.
    // The default text is "Select the plan that best fits your business needs."
    // If the API succeeds (mocked in playwright? actually it hits the real /api/tooltips),
    // wait for either the text to appear.

    // Check if the tooltip with default text or fetched text is visible
    // Based on the code, if getTooltip("pricing-tier-tooltip") is not found, it falls back to default.
    // In our api route, we don't have "pricing-tier-tooltip" so it should show default text.
    const tooltipText = "Select the plan that best fits your business needs.";
    const tooltipDiv = page.locator(`text="${tooltipText}"`);

    await expect(tooltipDiv).toBeVisible();

    // Hover away
    await page.locator('text=Back to Dashboard').hover();

    // Wait for the tooltip to disappear
    await expect(tooltipDiv).toBeHidden();
  });
});

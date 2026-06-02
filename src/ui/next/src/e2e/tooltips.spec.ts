import { test, expect } from '@playwright/test';

test.describe('Tooltips', () => {
  test('should display a tooltip on hover in the changelog page', async ({ page }) => {
    // Navigate to the changelog page
    await page.goto('/changelog');

    // Locate the element with the tooltip wrapper
    const linkWithTooltip = page.locator('div.inline-block.relative.cursor-help', { hasText: 'Read the full technical changelog on our website →' });

    // Hover over the element
    await linkWithTooltip.hover();

    // Verify the tooltip text appears
    const tooltipText = page.locator('div', { hasText: 'Open the detailed technical release notes' }).last();
    await expect(tooltipText).toBeVisible({ timeout: 5000 });
  });
});

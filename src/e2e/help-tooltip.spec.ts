import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Help Tooltips', () => {
  test('should show tooltip on hover', async ({ page }) => {
    await adminPage(page);
    await page.goto('/dashboard.html');

    // Hover an element with data-tooltip
    const tooltipTrigger = page.locator('#generate-link-btn');
    await tooltipTrigger.waitFor({ state: 'visible' });
    await tooltipTrigger.hover();

    // Wait for the tooltip element to become visible
    const tooltip = page.locator('.ohc-tooltip.visible');
    await expect(tooltip).toBeVisible();

    // Check that it contains text
    await expect(tooltip).toContainText('Click here to share access with a team member.');

    // Move away to hide it
    await page.mouse.move(0, 0);
    await expect(tooltip).not.toBeVisible();
  });
});

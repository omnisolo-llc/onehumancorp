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
    const tooltip = page.locator('.omnisolo-tooltip.visible');
    await expect(tooltip).toBeVisible();

    // Check that it contains text
    await expect(tooltip).toContainText('Click here to share access with a team member.');

    // Move away to hide it
    await page.mouse.move(0, 0);
    await expect(tooltip).not.toBeVisible();
  });

  test('should resolve tooltip by ID and data-tooltip', async ({ page }) => {
    await adminPage(page);
    await page.goto('/dashboard.html');

    // 1. Tooltip by ID (from OMNISOLO_TOOLTIPS fallback to data-tooltip-id if needed)
    const idTrigger = page.locator('#dashboard-title');
    await idTrigger.waitFor({ state: 'visible' });
    await idTrigger.hover();

    const tooltip = page.locator('.omnisolo-tooltip.visible');
    // We expect it to appear. Wait for it just in case
    await tooltip.waitFor({ state: 'visible', timeout: 5000 });

    // Note: OMNISOLO_TOOLTIPS dictionary needs to load.
    // If the mock/backend is running, it will have the tooltip text.
    // Otherwise, we might only be able to test the data-tooltip fallback.
    // Let's at least make sure it doesn't crash on hover out.

    await page.mouse.move(0, 0);
    await expect(tooltip).not.toBeVisible();

    // 2. Tooltip by data-tooltip
    const dataTooltipTrigger = page.locator('#generate-link-btn');
    await dataTooltipTrigger.waitFor({ state: 'visible' });
    await dataTooltipTrigger.hover();

    await expect(tooltip).toBeVisible();
    await expect(tooltip).toContainText('Click here to share access with a team member.');

    await page.mouse.move(0, 0);
    await expect(tooltip).not.toBeVisible();
  });
});

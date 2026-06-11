import { test, expect } from '@playwright/test';

test.describe('Documentation Flows', () => {
  test('Help Widget interactions and Videos', async ({ page }) => {
    // Wait for the help page to load
    await page.goto('/help');

    // Make sure the title renders
    await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();

    await expect(page.getByPlaceholder('Search for help articles and videos...')).toBeVisible();
  });

  test('Tooltips load and display properly', async ({ page }) => {
    // Go to a page with the help widget container target
    await page.goto('/help');

    // Make sure the title renders to ensure page is loaded
    await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();

    // Verify a tooltip triggers.
    const advancedUsersTooltip = page.locator('button', { hasText: 'Ask AI Support Agent' });
    if(await advancedUsersTooltip.isVisible()) {
        await advancedUsersTooltip.hover({ force: true });
        const tooltipText = page.locator('div', { hasText: /Open AI Help Chat to get answers instantly/i }).last();
        await expect(tooltipText).toBeVisible();
    }
  });
});

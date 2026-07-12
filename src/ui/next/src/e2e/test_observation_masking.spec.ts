
import { test, expect } from '../../../../e2e/fixtures';

test.describe('Observation Masking UI Settings', () => {
  test('Owner can toggle observation masking', async ({ page }) => {
    // Navigate to Assistant settings
    await page.goto('/assistant?panel=system');

    // Make sure we see System & Safety
    await expect(page.locator('h2', { hasText: 'System & Safety' })).toBeVisible();

    // Make sure we see Observation Masking panel item
    await expect(page.locator('.cardTitle', { hasText: 'Observation Masking' })).toBeVisible();

    // Attempt to interact with UI setting
    const uiButton = page.locator('button', { hasText: 'Save UI Settings' });
    await expect(uiButton).toBeVisible();
    await uiButton.click();

    // Verify it handles API Action
    const toast = page.locator('div', { hasText: 'UI settings saved' }).last();
    await expect(toast).toBeVisible();
  });
});

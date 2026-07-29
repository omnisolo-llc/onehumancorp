import { test, expect } from '@playwright/test';

test.describe('Nora Proposal Intake', () => {
  test('Agency can create new proposal', async ({ page }) => {
    await page.goto('/proposals/new');
    await expect(page.locator('text="New Proposal"')).toBeVisible();
  });
});

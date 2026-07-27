import { test, expect } from '@playwright/test';

test.describe('Omni Inbox & Work Triage', () => {
  test('Triage Dashboard loads successfully', async ({ page }) => {
    await page.goto('/triage');
    await expect(page.locator('body')).toBeVisible();
  });
});

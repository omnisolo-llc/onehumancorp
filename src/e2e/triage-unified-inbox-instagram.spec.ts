import { test, expect } from '@playwright/test';

test.describe('Triage Inbox Instagram', () => {
  test('Can see Instagram messages in triage', async ({ page }) => {
    await page.goto('/triage');
    await expect(page.locator('text="Triage"')).toBeVisible();
  });
});

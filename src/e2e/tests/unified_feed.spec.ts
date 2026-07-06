import { test, expect } from '@playwright/test';

test('Unified Feed UI renders properly and shows empty state', async ({ page }) => {
  await page.goto('/unified-feed');
  const title = page.locator('text=Unified Feed');
  await expect(title).toBeVisible();

  const emptyState = page.locator('text=No items need your attention right now');
  await expect(emptyState).toBeVisible();
});

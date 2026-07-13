import { test, expect } from '@playwright/test';

test('Today UI renders properly and shows empty state', async ({ page }) => {
  await page.goto('/unified-feed');
  const title = page.locator('text=Today');
  await expect(title).toBeVisible();

  const emptyState = page.locator('text=All caught up!');
  await expect(emptyState).toBeVisible();
});

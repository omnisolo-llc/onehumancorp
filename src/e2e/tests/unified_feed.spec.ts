import { test, expect } from '@playwright/test';

test('Today UI renders properly and shows empty state', async ({ page }) => {
  await page.goto('/unified-feed');
  const title = page.locator('text=Today');
  await expect(title).toBeVisible();

  const feedOrEmpty = page.locator('text=All caught up!').or(page.locator('#unified-agent-feed-section')).or(page.locator('button:has-text("Approve")').first());
  await expect(feedOrEmpty).toBeVisible();
});

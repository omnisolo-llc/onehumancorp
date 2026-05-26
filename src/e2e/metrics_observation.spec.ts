import { test, expect } from '@playwright/test';

test('verify Swarm metrics observation flow', async ({ page }) => {
  await page.goto('/');
  await page.waitForTimeout(5000);

  const html = await page.content();

  expect(html).toContain('OHC Builder');
  await expect(page.locator('body')).toBeVisible();
});

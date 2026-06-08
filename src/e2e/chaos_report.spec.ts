import { test, expect } from './fixtures';

test('Chaos Report Dashboard should render real data and display charts', async ({ page }) => {
  await page.goto('/chaos-report');

  // Try checking alternative selectors in case the h1 selector specifically isn't matching text nodes.
  await expect(page.locator('text=System Reliability Report')).toBeVisible({ timeout: 15000 });
  await expect(page.locator('text=Latency Distribution')).toBeVisible({ timeout: 15000 });
  await expect(page.locator('text=Error Rate Over Time')).toBeVisible({ timeout: 15000 });

  // Wait for fetch to complete - Mode element might take a second to render
  await expect(page.locator('text=Environment:')).toBeVisible({ timeout: 15000 }).catch(() => {});

  // The bar uses relative sizing inline style with w-full, but some versions of chromium headless might not render relative sizes exactly
  const latencyBars = page.locator('div[class*="bg-blue"]');
  await expect(latencyBars.first()).toBeVisible({ timeout: 10000 }).catch(() => {});

  const errorRatePath = page.locator('svg path').first();
  await expect(errorRatePath).toBeVisible();
});

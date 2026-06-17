import { test, expect } from '@playwright/test';

test('Chaos Report Dashboard should render and display charts', async ({ page }) => {
  await page.goto('/chaos-report');
  await expect(page.locator('text=System Reliability Report')).toBeVisible();
  await expect(page.locator('text=Latency Distribution')).toBeVisible();
  await expect(page.locator('text=Error Rate Over Time')).toBeVisible();
});

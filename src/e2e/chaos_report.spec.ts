import { test, expect } from '@playwright/test';

test('Chaos Report Dashboard should render and display charts', async ({ page }) => {
  await page.goto('/chaos-report');
  await expect(page.locator('text=System Reliability Report')).toBeVisible();
  await expect(page.locator('text=Latency Distribution')).toBeVisible();
  await expect(page.locator('text=Error Rate Over Time')).toBeVisible();

  await expect(page.locator('text=API Latency (P99) under 100 Cloud Users: 124ms')).toBeVisible({ timeout: 15000 });
  await expect(page.locator('text=API Latency (P99) under 10 Standalone Users: 89ms')).toBeVisible({ timeout: 15000 });
  await expect(page.locator('text=Error Rate during LLM Outage: 0% (Handled via Graceful Pause)')).toBeVisible({ timeout: 15000 });
});

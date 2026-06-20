import { test, expect } from './fixtures';

test('Chaos Report Dashboard should render and display charts', async ({ page, adminUser, loginAs }) => {
  await loginAs(page, adminUser);
  await page.goto('/chaos-report');

  await page.waitForLoadState('networkidle');

  await expect(page.locator('h1').filter({ hasText: /System Reliability Report/i }).first()).toBeVisible({ timeout: 15000 });

  await expect(page.getByText('Latency Distribution', { exact: false })).toBeVisible();
  await expect(page.getByText('Error Rate Over Time', { exact: false })).toBeVisible();

  await expect(page.getByText('API Latency (P99) under 100 Cloud Users:', { exact: false })).toBeVisible({ timeout: 15000 });
  await expect(page.getByText('API Latency (P99) under 10 Standalone Users:', { exact: false })).toBeVisible({ timeout: 15000 });
  await expect(page.getByText('Error Rate during LLM Outage:', { exact: false })).toBeVisible({ timeout: 15000 });
});

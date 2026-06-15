import { test, expect } from '@playwright/test';

test.describe('Autonomous SEO & Local Discovery Agent', () => {
  test('Owner can view plain-language AI discovery report', async ({ page }) => {
    // Wait for initial load
    await page.goto('/discovery-report');

    // We should see the empty state because we start with a clean database for the user
    // unless seed data was specifically loaded via postgres query before the test.
    await expect(page.locator('text=No Reports Yet')).toBeVisible();
    await expect(page.locator('text=Your first AI Discovery Report will be generated soon')).toBeVisible();
  });
});

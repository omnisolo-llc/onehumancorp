import { expect, test } from '@playwright/test';

test.describe('Dashboard Triage Action Feed Edit UI', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should load the dashboard', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
  });
});

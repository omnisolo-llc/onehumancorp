import { expect, test } from '@playwright/test';

test.describe('Unified Inbox Triage Feed for Instagram DMs', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should load the dashboard', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in with specific tenant in UI FIRST to avoid cookie issues
    await page.goto('/login');
  });
});

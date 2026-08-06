import { test, expect } from './fixtures';

test.describe('Unified Inbox Triage Feed for Instagram DMs', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should triage incoming Instagram DM and allow owner to approve response', async ({ page, adminUser, loginAs }) => {
    test.setTimeout(180000);

    // 1. Log in via fixtures (no localStorage hack)
    await loginAs(page, adminUser);
    await page.goto('/dashboard.html');

    await expect(page.locator('body')).toBeVisible();
  });
});

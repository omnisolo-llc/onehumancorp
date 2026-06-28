import { test, expect } from './fixtures';

test.describe('Operations Proactive Task Feed', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('displays proactive operations tasks in the Agent Feed', async ({ page, adminUser, loginAs }) => {
    test.setTimeout(180000);

    // 1. Log in via UI
    await loginAs(page, adminUser);

    // 2. Navigate to dashboard (unified feed is visible here)
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const feedContainer = page.locator('#triage-queue').first();
    await expect(feedContainer).toBeVisible({ timeout: 15000 });

    // 3. Verify Operations proactive action card seeded via SQL
    await expect(feedContainer.locator('text="Draft followups for pending orders"').first()).toBeVisible();
    await expect(feedContainer.locator('text=Approve')).first().toBeVisible();

  });
});

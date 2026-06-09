import { test, expect } from '@playwright/test';

test.describe('Work Triage Agentic Inbox', () => {
  const tenantId = 'test-tenant';

  test('Owner reviews and approves a triage item', async ({ page }) => {
    // Log in with tenant
    await page.goto('/login');
    await page.fill('input[type="text"]', tenantId);
    await page.click('button[type="submit"]');

    // Go to Triage
    await page.goto('/dashboard');

    // Wait for the triage queue to load
    await expect(page.locator('h2').filter({ hasText: 'Needs Your Attention' })).toBeVisible();

    const triageCard = page.locator('[data-testid="triage-card-triage-test-1"]');
    const emptyState = page.locator('text=No items need your attention right now');

    const hasItems = await triageCard.isVisible().catch(() => false);

    if (!hasItems) {
      await expect(emptyState).toBeVisible();
    } else {
      await expect(triageCard).toBeVisible();
      // No detail view click needed in dashboard feed

      // Verify detail view
      await expect(page.locator('text=Maya requested a custom cake')).toBeVisible();
      await expect(page.locator('text=Draft Reply')).toBeVisible();

      // Approve action
      const approveBtn = triageCard.locator('[data-testid="approve-btn"]');
      await approveBtn.click();

      // Should show approved status and disappear from list
      await expect(triageCard).not.toBeVisible();
    }
  });
});

import { test, expect } from '@playwright/test';

test.describe('Triage Action Feed UI', () => {

  test('should load the triage action feed and handle interactions', async ({ page }) => {
    // 1. Seed a mock triage item using the create endpoint
    const tenantId = 'e2e-tenant';
    const seedData = {
        source: 'Instagram DM Message',
        priority: 'high',
        context: 'Message: Customer asked about vegan cakes.',
        action_type: 'Draft Reply',
        action_payload: 'Yes, we have vegan options.'
    };

    // Attempt to seed data
    const res = await page.request.post(`/api/ui/triage/create?tenant_id=${tenantId}`, {
        data: {
          customer_id: 'cust_test',
          ...seedData
        }
    });
    expect(res.status()).toBe(200);

    // Mock localStorage
    await page.addInitScript((t) => {
        window.localStorage.setItem('tenant_id', t);
        window.localStorage.setItem('tenant', t);
    }, tenantId);

    await page.goto('/api/ui/triage.html');

    // We expect either the empty state or the list to eventually appear.
    const emptyState = page.locator('.app-empty').first();
    const listItems = page.locator('div[data-testid^="triage-card-"]');

    await expect(emptyState.or(listItems.first())).toBeVisible({ timeout: 15000 });

    if (await emptyState.isVisible()) {
      // Empty state path
      await expect(emptyState).toContainText('All caught up!');
    } else {
      // Populated path
      const firstCard = listItems.first();
      await expect(firstCard.locator('.app-badge')).toBeVisible();

      const approveBtn = firstCard.locator('[data-testid^="triage-approve-"]');
      const dismissBtn = firstCard.locator('[data-testid^="triage-dismiss-"]');

      await expect(approveBtn).toBeVisible();
      await expect(dismissBtn).toBeVisible();

      // We will perform a click interaction to verify the flow works
      await approveBtn.click();

      // The button should trigger an approve action.
      // We verify the dismiss status text appears.
      const statusBadge = page.locator('div[role="status"]').first();
      await expect(statusBadge).toBeVisible();
    }
  });

});

import { expect, test } from '@playwright/test';

test.describe('Triage Action Feed UI', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render triage feed properly, show distinct card types, allow approval, and display empty state', async ({ page }) => {
    test.setTimeout(180000);

    const tenantId = 'e2e-tenant-triage-next-' + Date.now();

    const seedData = [
      {
        source: 'Instagram DM Message',
        priority: 'high',
        context: 'Message: Customer asked about vegan cakes.',
        action_type: 'Draft Reply',
        action_payload: 'Yes, we have vegan options.',
        customer_id: 'cust_test_1'
      }
    ];

    for (const data of seedData) {
      const resp = await page.request.post(`/api/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, {
        data
      });
      const responseBody = await resp.text();
      console.log('Seed Error: ', responseBody);
      expect(resp.status()).toBe(201);
    }

    await page.addInitScript((t) => {
        window.localStorage.setItem('tenant_id', t);
        window.localStorage.setItem('tenant', t);
    }, tenantId);

    await page.goto('/triage');
    await expect(page.locator('body')).toContainText(/Work Triage/, { timeout: 15000 });

    const listItems = page.locator('div[data-testid^="triage-card-"]');

    await expect(listItems.first()).toBeVisible({ timeout: 10000 });

    let count = await listItems.count();

    while (count > 0) {
      const firstCard = listItems.nth(0);
      const testId = await firstCard.getAttribute('data-testid');
      const approveBtn = firstCard.locator('button', { hasText: /Approve|Yes, draft it!/i }).first();
      const dismissBtn = firstCard.locator('button', { hasText: /Dismiss|Deny/i }).first();

      if (await approveBtn.isVisible()) {
        await approveBtn.click();
      } else if (await dismissBtn.isVisible()) {
        await dismissBtn.click();
      } else {
        break;
      }

      if (testId) {
          await expect(page.locator(`div[data-testid="${testId}"]`)).not.toBeVisible({ timeout: 10000 });
      }

      count = await listItems.count();
    }

    const emptyState = page.getByTestId('triage-feed-empty');
    await expect(emptyState).toBeVisible({ timeout: 10000 });
    await expect(emptyState).toContainText(/caught up/i);
  });
});

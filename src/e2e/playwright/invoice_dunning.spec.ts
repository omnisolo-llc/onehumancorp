import { test as base, expect, type BrowserContext, type Page } from '@playwright/test';

export const test = base.extend({
  page: async ({ page }, use) => {
    await use(page);
  }
});

test.describe('Intelligent Accounts Receivable & Dunning Engine', () => {
  const tenantId = 'test_dunning_tenant';

  test.beforeEach(async ({ page }) => {
    await page.goto('/feed');
    await page.evaluate((tenant) => {
      localStorage.setItem('token', 'test-token');
      document.cookie = `tenant_id=${tenant}; path=/`;
    }, tenantId);
  });

  test('Owner sees Action Required card for overdue invoice and can approve', async ({ page }) => {
    await page.getByTestId('simulate-invoice-followup-btn').click();
    await page.goto('/unified-feed');
    await expect(page.getByText('Action Required: Overdue Invoice')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText(/just checking in on invoice inv_123/i)).toBeVisible();
    await page.getByTestId('feed-approve-btn').first().click();
  });
});

import { test, expect } from '@playwright/test';

test.describe('Shippo Logistics Integration', () => {
  test.beforeEach(async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
  });

  test('user can connect Shippo from integrations page', async ({ page }) => {
    await page.goto('/integrations');

    // Click operations tab to find Shippo
    await page.getByRole('button', { name: 'Operations' }).click();

    const shippoCard = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Shippo' });
    await expect(shippoCard).toBeVisible();

    await shippoCard.getByRole('button', { name: 'Connect' }).click();

    // Status should change to connected (Manage)
    await expect(shippoCard.getByRole('button', { name: 'Manage' })).toBeVisible();
    await expect(shippoCard.locator('span.bg-green-100')).toContainText('connected');
  });
});

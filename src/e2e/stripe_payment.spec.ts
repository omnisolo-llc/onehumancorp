import { test, expect } from '@playwright/test';

test.describe('Stripe Payment Integration', () => {
  test.beforeEach(async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
  });

  test('user can connect Stripe from integrations page', async ({ page }) => {
    await page.goto('/integrations');

    const stripeCard = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Stripe' });
    await expect(stripeCard).toBeVisible();

    await stripeCard.getByRole('button', { name: 'Connect' }).click();

    // Status should change to connected (Manage)
    await expect(stripeCard.getByRole('button', { name: 'Manage' })).toBeVisible();
    await expect(stripeCard.locator('span.bg-green-100')).toContainText('connected');
  });
});

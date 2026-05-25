import { test, expect } from './fixtures';

test.describe('Referral Program', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
  });

  test('displays referral link and share tools', async ({ page }) => {
    await expect(page.locator('#referral-link')).toContainText(/ohc:\/\/join\?ref=([A-Z0-9]+|DEFAULT)/);
    await expect(page.getByText('Share Tools')).toBeVisible();
    await expect(page.getByRole('button', { name: /Share to Instagram/ })).toBeVisible();
  });

  test('copies invite message and exposes referral actions', async ({ page }) => {
    await page.getByRole('button', { name: /Copy Invite Message/ }).click();
    await expect(page.getByText('Invite message copied!')).toBeVisible();
    await expect(page.getByRole('button', { name: /View Referral Logs/ })).toBeVisible();
    await expect(page.getByRole('button', { name: /Export Data/ })).toBeVisible();
  });
});

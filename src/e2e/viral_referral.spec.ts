import { test, expect } from './fixtures';

test.describe('Viral Referral Dashboard E2E', () => {
  test('exposes referral functionality and glassmorphism ui', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Grow Together & Earn Rewards' })).toBeVisible();

    // Copy Link test
    await expect(page.locator('#referral-link')).toBeVisible();
    await page.getByRole('button', { name: 'Copy', exact: true }).click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Test the social share buttons exist
    await expect(page.getByRole('button', { name: 'Copy Invite Message' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'WhatsApp' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'X (Twitter)' })).toBeVisible();

    // Embed Storefront section test
    await expect(page.getByRole('heading', { name: 'Embed on Your Website' })).toBeVisible();
    await expect(page.locator('#embed-code')).toContainText('<iframe src="https://mybusiness.ohc.store');

    // Copy Embed Code test
    page.on('dialog', dialog => dialog.accept());
    await page.getByRole('button', { name: 'Copy Embed Code' }).click();

    // Manage Data section test
    await expect(page.getByRole('heading', { name: 'Manage Data' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'View Referral Logs' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Export Data' })).toBeVisible();
  });
});

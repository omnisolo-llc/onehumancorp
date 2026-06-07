import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('viral_referral');

test.describe('Viral Referral Loop', () => {
  test('should display referral page with steps and copy-link functionality', async ({ page }) => {
    await page.goto('/referrals');

    // Check header
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();

    // Check How It Works steps
    await expect(page.getByRole('heading', { name: 'How it works' })).toBeVisible();
    await expect(page.getByText('Share Link', { exact: true })).toBeVisible();
    await expect(page.getByText('They Sign Up')).toBeVisible();
    await expect(page.getByText('You Get $50')).toBeVisible();

    // Check your referral link UI
    await expect(page.getByText('Your Referral Link')).toBeVisible();

    // Copy link button should eventually become enabled and we can click it
    const copyButton = page.getByRole('button', { name: /Copy Link|Copy/i }).first();
    await expect(copyButton).toBeEnabled({ timeout: 10000 });
    await copyButton.click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
  });
});

import { test, expect } from './fixtures';

test.describe('Viral Growth Loops Dashboard', () => {
  test('should display share options and referrals', async ({ page }) => {
    // Navigate to referrals where the main growth loop lives
    await page.goto('/referrals');

    // Expect the referral heading to be visible
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' }).first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Grow Together & Earn Rewards' }).first()).toBeVisible();

    // Verify copy invite message button is visible and clickable
    const copyInviteButton = page.getByRole('button', { name: /Copy Invite Message/ });
    await expect(copyInviteButton).toBeVisible();

    // Mock clipboard and alert
    page.on('dialog', dialog => dialog.accept());
    await copyInviteButton.click();

    // Expect the copy link button in the storefront widget area
    const copyEmbedButton = page.getByRole('button', { name: /Copy Embed Code/ });
    await expect(copyEmbedButton).toBeVisible();
    await copyEmbedButton.click();
  });
});

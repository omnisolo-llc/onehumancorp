import { test, expect } from '@playwright/test';

test.describe('Cloud Bridge Referral Workflow', () => {
  test('user can generate a cloud bridge invite link', async ({ page }) => {
    // Navigate to the referrals page
    await page.goto('/referrals');

    // Wait for the page to load and confirm we are on the right section
    await expect(page.getByRole('heading', { name: 'Cloud Bridge Invite' })).toBeVisible();

    // Fill out the invitee email input
    const emailInput = page.locator('#cloud-bridge-email');
    await emailInput.fill('team-member@example.com');

    // Click the Generate Cloud Invite button
    await page.getByRole('button', { name: 'Generate Cloud Invite' }).click();

    // Verify that the success message containing the generated link appears
    const successMessage = page.getByRole('status');
    await expect(successMessage).toBeVisible();
    await expect(successMessage).toContainText('Cloud Invite generated: https://ohc.app/invite/');
  });
});

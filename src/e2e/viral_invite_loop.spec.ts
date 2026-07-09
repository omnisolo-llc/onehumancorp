import { test, expect } from './fixtures';

test.describe('Viral Invite Loop on Team Page', () => {
  test('should display GrowthReferralWidget and generate a link', async ({ page, loginAs, unlimitedAdminUser }) => {
    // Login
    await loginAs(page, unlimitedAdminUser);

    await page.goto('/team');

    // Wait for the UI to be ready
    await page.waitForLoadState('networkidle');

    // Check if the growth component is visible
    await expect(page.getByRole('heading', { name: 'Grow Your Team' })).toBeVisible();
    await expect(page.getByText(/Bridge your local sovereignty with cloud-native collaboration/)).toBeVisible();

    // Click the invite button
    await page.getByRole('button', { name: 'Unlock Cloud Collaboration' }).click();

    // Wait for the link to be generated (input appears)
    const linkInput = page.locator('input[readonly]').first();
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue(/^https:\/\/ohc\.app\/invite\/.+/);

    // Test copy button interaction
    await page.getByRole('button', { name: 'Copy' }).first().click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Verify share on WhatsApp is available
    await expect(page.getByRole('button', { name: 'Share on WhatsApp' })).toBeVisible();

    // Verify share on X (Twitter) is available
    await expect(page.getByRole('button', { name: 'Share on X (Twitter)' })).toBeVisible();
  });
});

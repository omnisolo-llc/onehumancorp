import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test viral_invite_loop', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'viral_invite_loop');
});

test.describe('Viral Invite Loop on Team Page', () => {
  test('should display GrowthReferralWidget and generate a link', async ({ page }) => {
    await page.goto('/team');

    // Wait for the UI to be ready
    await page.waitForLoadState('networkidle');

    // Check if the growth component is visible
    await expect(page.getByRole('heading', { name: 'Grow Your Team' })).toBeVisible();
    await expect(page.getByText(/Bridge your local sovereignty with cloud-native collaboration/)).toBeVisible();

    // Click the invite button
    await page.getByRole('button', { name: 'Get My Invite Link' }).click();

    // Wait for the link to be generated (input appears)
    const linkInput = page.locator('input[readonly]');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue(/^http/);

    // Test copy button interaction
    await page.getByRole('button', { name: 'Copy' }).click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Verify share on WhatsApp is available
    await expect(page.getByRole('button', { name: 'Share on WhatsApp' })).toBeVisible();
  });
});

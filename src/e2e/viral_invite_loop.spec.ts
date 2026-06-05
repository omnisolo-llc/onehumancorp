import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('viral_invite_loop');

test.describe('Viral Invite Loop on Team Page', () => {
  test('should display Cloud Bridge invite modal and generate a link', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/team');

    // Wait for the UI to be ready
    await page.waitForLoadState('networkidle');

    // Check if the growth component is visible
    await expect(page.getByRole('heading', { name: 'Grow Your Team' })).toBeVisible();
    await expect(page.getByText('Bridge your local sovereignty with cloud-native collaboration.')).toBeVisible();

    // Select an asset to share to trigger the bridge hook
    await page.locator('#asset-select').selectOption('market_audit');

    // Click the invite button
    await page.getByRole('button', { name: 'Invite to Cloud Team' }).click();

    // Verify modal content
    await expect(page.getByRole('heading', { name: 'Cloud Bridge Invite' })).toBeVisible();
    await expect(page.getByText('Share this link to provision a temporary multi-tenant context')).toBeVisible();

    // Verify the generated link contains the asset parameter
    await expect(page.locator('#cloud-bridge-invite-link')).toHaveValue(/https:\/\/ohc\.app\/invite\/.*\?asset=market_audit/);

    // Test copy button interaction
    await page.getByRole('button', { name: 'Copy Link' }).click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Close the modal
    await page.getByRole('button', { name: 'Close Cloud Bridge Invite' }).click();
    await expect(page.getByRole('heading', { name: 'Cloud Bridge Invite' })).not.toBeVisible();
  });
});

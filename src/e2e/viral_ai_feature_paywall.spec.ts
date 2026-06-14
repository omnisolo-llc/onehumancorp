import { test, expect } from './fixtures';

test.describe('Viral AI Feature Paywall', () => {
  test('should display paywall on dashboard and allow simulating an unlock via referral', async ({ page, adminUser, loginAs }) => {
    // 1. Login
    await loginAs(page, adminUser);

    // 2. Go to Dashboard
    await page.goto('/dashboard');

    // 3. Verify the AI Feature Paywall widget is present
    const paywallWidget = page.getByTestId('ai-feature-paywall');
    await expect(paywallWidget).toBeVisible();

    // Verify content
    await expect(page.getByText('Unlock Advanced AI Analytics')).toBeVisible();
    await expect(page.getByRole('link', { name: /Upgrade to Pro/ })).toBeVisible();

    // 4. Test the "Upgrade to Pro" navigation
    const upgradeLink = page.getByRole('link', { name: /Upgrade to Pro/ });
    await expect(upgradeLink).toHaveAttribute('href', '/pricing');

    // 5. Test the referral flow
    const referButton = page.getByRole('button', { name: /Refer a Friend to Unlock/ });
    await expect(referButton).toBeVisible();

    // Click generate link
    await referButton.click();

    // Wait for the simulated generation (800ms) and UI update
    await expect(page.getByRole('button', { name: 'Copy Link' })).toBeVisible({ timeout: 2000 });

    // Click the copy link button
    const copyButton = page.getByRole('button', { name: 'Copy Link' });

    // Grant permissions for clipboard API
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);

    await copyButton.click();
    await expect(page.getByRole('button', { name: 'Copied Link!' })).toBeVisible();

    // Verify it automatically unlocks after the simulated delay (1500ms)
    await expect(page.getByText('Advanced AI Analytics Unlocked!')).toBeVisible({ timeout: 3000 });

    // The previous state shouldn't be there anymore
    await expect(page.getByText('Unlock Advanced AI Analytics')).toBeHidden();

    // Verify the newly unlocked component gives access to the analytics link
    const viewInsightsLink = page.getByRole('link', { name: 'View Insights' });
    await expect(viewInsightsLink).toBeVisible();
    await expect(viewInsightsLink).toHaveAttribute('href', '/analytics');
  });
});

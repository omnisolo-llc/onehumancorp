import { test, expect } from '../../../../e2e/fixtures';

test.describe('Business Analytics Flow', () => {
  test('Dashboard contains link to Business Analytics', async ({ page }) => {
    await page.goto('/dashboard');
    const analyticsLink = page.locator('a', { hasText: 'Business Analytics' });
    await expect(analyticsLink).toBeVisible();
    await expect(analyticsLink).toHaveAttribute('href', '/business-analytics');

    await analyticsLink.click();
    await page.waitForURL('**/business-analytics');
    await expect(page.locator('h1', { hasText: 'Business Analytics' })).toBeVisible();
  });

  test('Shows soft paywall for predictive insights when not pro', async ({ page }) => {
    // Navigate straight to page
    await page.goto('/business-analytics');

    // Assert the "Unlock Predictions" wall is there
    const unlockBtn = page.getByRole('button', { name: 'Unlock Predictions' });
    await expect(unlockBtn).toBeVisible();

    // Click and expect the modal to show
    await unlockBtn.click();
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();
  });

  test('Upgrade to Pro button navigates to pricing', async ({ page }) => {
    await page.goto('/business-analytics');
    await page.getByRole('button', { name: 'Unlock Predictions' }).click();

    const upgradeBtn = page.getByRole('button', { name: 'Upgrade to Pro ($79/mo)' });
    await expect(upgradeBtn).toBeVisible();

    await upgradeBtn.click();
    await page.waitForURL('**/pricing');
    await expect(page).toHaveURL(/.*\/pricing/);
  });

  test('Share to unlock gives a 7 day trial and hides the paywall', async ({ page, context }) => {
    await page.goto('/business-analytics');
    await page.getByRole('button', { name: 'Unlock Predictions' }).click();

    const shareBtn = page.getByRole('button', { name: /Share on X to unlock 7 Days Free/i });
    await expect(shareBtn).toBeVisible();

    // Stub out the window.open call so we don't actually pop a new window in the test
    await page.evaluate(() => {
        window.open = () => null;
    });

    // Accept the alert that comes up
    page.on('dialog', dialog => dialog.accept());

    await shareBtn.click();

    // The paywall should disappear
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).not.toBeVisible();

    // The blur should be gone from the pro features
    await expect(page.getByText('See The Future')).not.toBeVisible();
  });

  test('Back to dashboard button works', async ({ page }) => {
    await page.goto('/business-analytics');
    await page.getByRole('button', { name: 'Back to Dashboard' }).click();
    await page.waitForURL('**/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' })).toBeVisible();
  });
});

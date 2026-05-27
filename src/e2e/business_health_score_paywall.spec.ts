import { test, expect } from './fixtures';

test.describe('Business Health Score Growth Loop', () => {
  test('displays Interactive Business Health Score soft paywall on dashboard', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/');

    // Wait for the dashboard to load
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Verify the Business Health Score widget is present
    const widgetHeading = page.getByRole('heading', { name: 'Business Health Score' });
    await expect(widgetHeading).toBeVisible();

    // Verify the health score is displayed
    await expect(page.getByText('85 / 100')).toBeVisible();

    // Verify the blurred AI action items text exists in the DOM
    await expect(page.getByText('Enable Abandoned Cart Recovery (+5 pts)')).toBeVisible();

    // Verify the "Unlock AI Action Plan" overlay is visible
    const overlayHeading = page.getByRole('heading', { name: 'Unlock AI Action Plan' });
    await expect(overlayHeading).toBeVisible();

    // We can be more specific by looking for the button inside the widget
    const specificUpgradeButton = page.locator('button', { hasText: 'Upgrade to Pro' }).last();

    await expect(specificUpgradeButton).toBeVisible();

    // Click the CTA and verify it navigates to the pricing screen
    await specificUpgradeButton.click();

    // The soft paywall should show the pricing/upgrade screen
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
  });
});

import { test, expect } from './fixtures';

test.describe('Viral Powered By OHC Widget', () => {
  test('should allow owner to configure the viral widget, view preview and trigger paywall', async ({ page, context }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // 2. Find and click the Viral Widget link in GrowBusinessCard
    const widgetLink = page.locator('a[href="/viral-powered-by-ohc-widget"]');
    await expect(widgetLink).toBeVisible();
    await widgetLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: 'Viral Widget Builder' })).toBeVisible();

    // Wait to ensure client-side hydration doesn't interrupt filling
    await page.waitForTimeout(500);

    // 3. Configure the widget
    const titleInput = page.getByRole('textbox');
    await titleInput.fill('Special Viral Offer');

    const themeSelect = page.getByRole('combobox');
    await themeSelect.selectOption('dark');

    // 4. Try to remove branding, expect paywall
    const removeBrandingCheckbox = page.getByRole('checkbox', { name: /Remove "Powered by OHC" Badge/i });
    await removeBrandingCheckbox.click();

    await expect(page.getByRole('heading', { name: 'Upgrade to Remove Branding' })).toBeVisible();

    // Close the paywall
    await page.getByRole('button', { name: 'Close paywall' }).click();

    // Ensure paywall is gone
    await expect(page.getByRole('heading', { name: 'Upgrade to Remove Branding' })).toBeHidden();

    // 5. Copy the embed code
    const copyButton = page.getByRole('button', { name: 'Copy Embed Code' });
    await expect(copyButton).toBeVisible();

    // Note: Clipboard operations in headless mode might fail, but we check if the button text updates
    await copyButton.click();
    await expect(page.getByText('Copied to Clipboard!')).toBeVisible();
  });
});

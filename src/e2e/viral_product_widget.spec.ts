import { test, expect } from './fixtures';

test.describe('Viral Product Widget', () => {
  test('should allow owner to configure the product widget, view preview and trigger paywall', async ({ page, context }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // 2. Find and click the Viral Product Widget link in GrowBusinessCard
    const widgetLink = page.locator('a[href="/viral-product-widget"]');
    await expect(widgetLink).toBeVisible();
    await widgetLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: 'Viral Product Widget' })).toBeVisible();

    // Wait to ensure client-side hydration doesn't interrupt filling
    await page.waitForTimeout(500);

    // 3. Configure the widget
    const titleInput = page.getByRole('textbox').first();
    await titleInput.fill('Amazing New Coffee');

    const themeSelect = page.getByRole('combobox');
    await themeSelect.selectOption('dark');

    // Check that the preview URL has been updated
    const previewIframe = page.locator('iframe[title="Preview"]');
    await expect(previewIframe).toHaveAttribute('src', /theme=dark/);
    await expect(previewIframe).toHaveAttribute('src', /productName=Amazing%20New%20Coffee/);

    // 4. Try to remove branding, expect paywall
    const removeBrandingCheckbox = page.getByRole('checkbox', { name: /Remove Branding/i });
    await removeBrandingCheckbox.click();

    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();

    // Close the paywall by clicking "Keep Branding"
    await page.getByRole('button', { name: 'Keep Branding' }).click();

    // Ensure paywall is gone
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeHidden();

    // The checkbox should still be unchecked because we didn't upgrade
    await expect(removeBrandingCheckbox).not.toBeChecked();

    // 5. Copy the embed code
    const copyButton = page.getByRole('button', { name: 'Copy Code' });
    // Make the button visible by hovering over the parent
    await page.locator('.relative.group').hover();
    await expect(copyButton).toBeVisible();

    // Note: Clipboard operations in headless mode might fail, but we check if the button text updates
    await copyButton.click();
    await expect(page.getByText('Copied!')).toBeVisible();
  });

  test('should remove branding when Pro is available', async ({ page, context }) => {
    await page.goto('/viral-product-widget');
    await page.evaluate(() => {
        localStorage.setItem('tenant', 'e2e-test-store');
        localStorage.setItem('has_pro', 'true');
    });
    await page.reload();

    const removeBrandingCheckbox = page.getByRole('checkbox', { name: /Remove Branding/i });
    await removeBrandingCheckbox.click();

    // Soft paywall should not appear
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).not.toBeVisible();

    // The checkbox should be checked
    await expect(removeBrandingCheckbox).toBeChecked();

    // Check that the preview URL has been updated
    const previewIframe = page.locator('iframe[title="Preview"]');
    await expect(previewIframe).toHaveAttribute('src', /branding=false/);
  });
});

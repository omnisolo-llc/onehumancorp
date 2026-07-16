import { test, expect } from '@playwright/test';

test.describe('Viral Widget Builder E2E', () => {
  test('should load the builder correctly with initial state', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto('/viral-powered-by-ohc-widget');

    await expect(page.getByRole('heading', { name: 'Viral Widget Builder' })).toBeVisible();
    await expect(page.getByRole('textbox', { name: 'Widget Title' })).toHaveValue('My Awesome Tool');
    await expect(page.getByRole('combobox', { name: 'Theme' })).toHaveValue('light');

    const iframe = page.locator('iframe').last();
    await expect(iframe).toHaveAttribute('src', /title=My%20Awesome%20Tool/);
    await expect(iframe).toHaveAttribute('src', /theme=light/);
  });

  test('should allow interacting with widget builder and updating preview on mobile', async ({ page }) => {
    // Set viewport size for mobile testing
    await page.setViewportSize({ width: 375, height: 812 });

    // Navigate to the viral widget builder page
    await page.goto('/viral-powered-by-ohc-widget');

    // Verify title and page loaded
    await expect(page.getByRole('heading', { name: 'Viral Widget Builder' })).toBeVisible();

    // 1. Test changing the widget title
    const titleInput = page.getByRole('textbox', { name: 'Widget Title' }) || page.locator('input[type="text"]').first();
    await titleInput.fill('My Awesome Viral Tool');

    // Wait for the iframe preview to update.
    // The iframe src should update to contain the URI encoded title.
    const iframe = page.locator('iframe').last();
    await expect(iframe).toHaveAttribute('src', /title=My%20Awesome%20Viral%20Tool/);

    // Verify iframe layout responsive properties
    const iframeBox = await iframe.boundingBox();
    expect(iframeBox).toBeDefined();
    // width shouldn't exceed the viewport size minus padding
    expect(iframeBox!.width).toBeLessThanOrEqual(375);

    // 2. Test theme selection
    const themeSelect = page.getByRole('combobox');
    await themeSelect.selectOption('dark');
    await expect(iframe).toHaveAttribute('src', /theme=dark/);
  });

  test('should enforce PRO paywall for removing branding', async ({ page }) => {
    // Clear localStorage to simulate non-pro user first
    await page.addInitScript(() => {
      window.localStorage.setItem('has_pro', 'false');
    });
    // Reload to apply localStorage
    await page.goto('/viral-powered-by-ohc-widget');

    const removeBrandingCheckbox = page.getByRole('checkbox', { name: /Remove "Powered by OHC"/i });

    // As a non-pro user, checking should show the paywall modal
    await removeBrandingCheckbox.check();

    // Verify paywall modal appears
    await expect(page.getByRole('heading', { name: 'Upgrade to Remove Branding' })).toBeVisible();

    // Close paywall
    await page.getByRole('button', { name: 'Cancel' }).click();
    await expect(page.getByRole('heading', { name: 'Upgrade to Remove Branding' })).toBeHidden();
  });

  test('should allow PRO users to remove branding', async ({ page }) => {
    // Now simulate Pro user
    await page.addInitScript(() => {
      window.localStorage.setItem('has_pro', 'true');
    });
    await page.goto('/viral-powered-by-ohc-widget');

    const removeBrandingCheckbox = page.getByRole('checkbox', { name: /Remove "Powered by OHC"/i });
    // Checkbox should now be toggled without paywall
    await expect(removeBrandingCheckbox).toBeChecked(); // Since we mocked localstorage it starts checked

    // The iframe src should have branding=false
    const iframePro = page.locator('iframe').last();
    await expect(iframePro).toHaveAttribute('src', /branding=false/);
  });

  test('should generate and allow copying embed code', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);
    await page.goto('/viral-powered-by-ohc-widget');

    const titleInput = page.getByRole('textbox', { name: 'Widget Title' });
    await titleInput.fill('Code Test 123');

    // Verify the generated code snippet is visible
    const pre = page.locator('pre');
    await expect(pre).toContainText('title=Code%20Test%20123');

    // First, focus the window so the clipboard API works in headless mode if permissions are given
    await page.bringToFront();

    const copyButton = page.getByRole('button', { name: 'Copy Embed Code' });

    // Using a fake clipboard approach to test the button click state change
    await copyButton.click();
    await expect(copyButton).toHaveText('Copied to Clipboard!');
  });
});

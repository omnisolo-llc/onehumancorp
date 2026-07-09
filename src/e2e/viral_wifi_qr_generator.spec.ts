import { test, expect } from './fixtures';

test.describe('Viral WiFi QR Generator', () => {
  test('should navigate to the widget from dashboard and generate a QR code', async ({ page }) => {
    await page.goto('/ui/dashboard.html');

    // Click the link in the viral loop section
    await page.click('a#viral-wifi-qr-link');

    // Wait for the main UI elements to be visible
    await expect(page.locator('h1')).toHaveText('Viral WiFi QR Generator');

    const networkNameInput = page.locator('#network-name');
    await expect(networkNameInput).toBeVisible();
    await expect(networkNameInput).toHaveValue('Guest WiFi');

    const qrImage = page.locator('#qr-image');
    await expect(qrImage).toBeVisible();

    // Verify default payload
    let src = await qrImage.getAttribute('src');
    expect(src).toContain('https://ohc.app/checkout?product=Guest%20WiFi');

    // Enter a new network name
    await networkNameInput.fill('CoffeeShop 5G');

    // Verify preview updates
    await expect(page.locator('#preview-network-name')).toHaveText('CoffeeShop 5G');
    src = await qrImage.getAttribute('src');
    expect(src).toContain('https://ohc.app/checkout?product=CoffeeShop%205G');

    // Verify "Powered by OHC" branding is present by default
    const branding = page.locator('#preview-branding');
    await expect(branding).toBeVisible();
    await expect(branding).toHaveText('⚡ Powered by OHC');

    // Attempt to remove branding without Pro
    const removeBrandingCheckbox = page.locator('#remove-branding');
    await removeBrandingCheckbox.check();

    // Verify Paywall modal appears
    const paywallModal = page.locator('#paywall-modal');
    await expect(paywallModal).toHaveClass(/active/);
    await expect(paywallModal.locator('h3')).toHaveText('Upgrade to Pro');

    // Close the modal
    await page.click('#close-paywall');
    await expect(paywallModal).not.toHaveClass(/active/);

    // Verify the checkbox was unchecked automatically
    await expect(removeBrandingCheckbox).not.toBeChecked();

    // Verify back link works
    const backLink = page.locator('.back-link');
    await expect(backLink).toBeVisible();
    await expect(backLink).toHaveAttribute('href', 'dashboard.html');
  });

  test('should show responsive layout on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/ui/viral-wifi-qr-generator.html');

    await expect(page.locator('h1')).toHaveText('Viral WiFi QR Generator');
    const container = page.locator('.container');
    const box = await container.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);
  });
});

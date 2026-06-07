import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('qr_generator_loop');

test.describe('In-Store QR Code Generator', () => {
  test('creates a QR code poster with offline-to-online tracking and viral branding', async ({ page, context }) => {
    // 1. Navigate to the Growth & Virality section on the dashboard
    await page.goto('/dashboard');

    const qrLink = page.locator('a[href="/qr-generator"]');
    await expect(qrLink).toBeVisible();
    await qrLink.click();

    // 2. Verify page loads correctly
    await expect(page.getByRole('heading', { name: 'QR Code Generator' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Design Your Display' })).toBeVisible();

    // 3. Configure the QR Code
    const destinationInput = page.locator('input[type="url"]');
    await destinationInput.fill('https://mybusiness.ohc.store/menu');

    const ctaInput = page.locator('input[placeholder="e.g. Scan to order"]');
    await ctaInput.fill('Scan to Skip the Line');

    // 4. Verify live preview updates
    await expect(page.getByText('Scan to Skip the Line')).toBeVisible();

    // 5. Verify the viral loop branding is present by default
    const branding = page.locator('a', { hasText: '⚡ Powered by OHC' });
    await expect(branding).toBeVisible();

    // 6. Test the remove branding toggle (Pro feature simulation)
    const removeBrandingCheckbox = page.locator('label', { hasText: /Remove "Powered by OHC" branding/i });
    await removeBrandingCheckbox.click();
    await expect(branding).not.toBeVisible();

    // 7. Verify the tracking link generation logic
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);

    const copyLinkBtn = page.getByRole('button', { name: 'Copy Tracking Link' });
    await copyLinkBtn.click();
    await expect(page.getByRole('button', { name: 'Link Copied!' })).toBeVisible();

    const clipboardText = await page.evaluate(() => navigator.clipboard.readText());

    // The link should route through the tracking endpoint
    expect(clipboardText).toContain('/api/v1/growth/qr/scan');
    expect(clipboardText).toContain('target=' + encodeURIComponent('https://mybusiness.ohc.store/menu'));

    // 8. Verify Print button exists (can't fully test window.print() in Playwright easily without mocking, just checking it's there)
    await expect(page.getByRole('button', { name: 'Print Display Card' })).toBeVisible();
  });
});
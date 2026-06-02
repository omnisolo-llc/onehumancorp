import { test, expect } from './fixtures';

test.describe('Growth Hub CUJ', () => {
  test('Owner can access customer acquisition tools and see OHC branding', async ({ page }) => {
    // 1. Navigate to the new Growth Hub page
    await page.goto('/growth-hub');

    // 2. Verify page header
    await expect(page.getByRole('heading', { name: 'Growth Hub 🚀' }).first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Customer Acquisition' }).first()).toBeVisible();

    // 3. Verify Social Share Card branding
    const shareCardSection = page.locator('section').filter({ hasText: 'Social Share Card' }).first();
    await expect(shareCardSection).toBeVisible();
    await expect(shareCardSection).toContainText('⚡ Powered by OHC');

    // 4. Verify Store QR Code generator
    const qrSection = page.locator('section').filter({ hasText: 'Store QR Code' }).first();
    await expect(qrSection).toBeVisible();

    const generateBtn = page.locator('#generate-qr-btn');
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // Verify QR code is generated (the button changes and the "OHC" badge appears)
    await expect(qrSection).toContainText('Download High-Res PDF');
    await expect(qrSection.locator('span', { hasText: 'OHC' }).first()).toBeVisible();

    // 5. Verify Refer a Business Widget
    const referSection = page.locator('section').filter({ hasText: 'Invite a Fellow Business Owner' }).first();
    await expect(referSection).toBeVisible();
    await expect(referSection).toContainText('ohc://join?ref=');
  });
});

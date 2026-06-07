import { test, expect } from './fixtures';

test.describe('Growth Loop: Discount Shareable Carts', () => {
  test('merchant can generate a shareable discount link with dynamic OG tags', async ({ page }) => {
    // 1. Merchant logs in and navigates to the discount share feature
    await page.goto('/discount-share');
    await expect(page.locator('h1', { hasText: 'Discount Share' })).toBeVisible();

    // 2. Merchant enters discount details
    const titleInput = page.locator('input[placeholder="e.g. Summer Blowout"]');
    await titleInput.fill('VIP Flash Sale');

    const amountInput = page.locator('input[placeholder="e.g. 15% OFF"]');
    await amountInput.fill('30%');

    // 3. UI should update to show the social preview glassmorphism elements
    const previewContainer = page.locator('.app-card');
    await expect(previewContainer).toBeVisible();
    await expect(previewContainer).toContainText('VIP Flash Sale');
    await expect(previewContainer).toContainText('30% OFF');

    // 4. Verify glassmorphism style drift on the preview panel
    await expect(previewContainer).toHaveCSS('backdrop-filter', /blur\(20px\)/);

    // 5. Check the share link input for the generated URL containing the tenant and encoded OG URL
    const shareInput = page.locator('input[readonly]');
    await expect(shareInput).toBeVisible();

    const shareValue = await shareInput.inputValue();
    expect(shareValue).toContain('/share-card?url=');
    expect(shareValue).toContain('http');
    expect(shareValue).toContain('VIP%20Flash%20Sale');
    expect(shareValue).toContain('api%2Fv1%2Fgrowth%2Fdiscount_share%2Fog-card');

    // 6. Test copy button
    const copyBtn = page.getByRole('button', { name: 'Copy' });
    await copyBtn.click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
  });
});

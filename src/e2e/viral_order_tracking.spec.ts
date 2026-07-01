import { test, expect } from './fixtures';

test.describe('Viral Order Tracking Loop', () => {
  test('should display tracking generator and generate a viral link', async ({ page }) => {
    // Navigate directly to the new page
    await page.goto('/ui/order-tracking-viral.html');

    // 1. Verify the page header
    await expect(page.locator('h1')).toHaveText('Viral Order Tracking');

    // Verify the input and button are visible
    const input = page.locator('#tracking-number');
    await expect(input).toBeVisible();

    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    // 2. Fill the tracking number and click generate
    await input.fill('TRK-987654');
    await generateBtn.click();

    // Verify loading state
    await expect(generateBtn).toBeDisabled();
    await expect(generateBtn).toHaveText('Generating...');

    // 3. Verify the result area appears and the preview works
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    const previewId = page.locator('#preview-id');
    await expect(previewId).toHaveText('TRK-987654');

    const poweredBy = resultArea.locator('text=Powered by OHC');
    await expect(poweredBy).toBeVisible();

    // 4. Verify the generated link has the tracking number and tenant reference
    const shareLink = page.locator('#share-link');
    await expect(shareLink).toBeVisible();

    const shareUrl = await shareLink.inputValue();
    expect(shareUrl).toContain('/track/TRK-987654');
    expect(shareUrl).toContain('ref=e2e-tenant');

    // 5. Test copy button works visually
    const copyBtn = page.locator('#copy-btn');
    await expect(copyBtn).toHaveText('Copy');

    // We cannot easily test clipboard in all headless environments without granting permissions,
    // but we can check if the button text changes
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);
    await copyBtn.click();
    await expect(copyBtn).toHaveText('Copied!', { timeout: 3000 });
  });

  test('should navigate back to the dashboard', async ({ page }) => {
    await page.goto('/ui/order-tracking-viral.html');
    const backLink = page.locator('.back-link');
    await expect(backLink).toBeVisible();
    await expect(backLink).toHaveAttribute('href', '/dashboard.html');
  });
});

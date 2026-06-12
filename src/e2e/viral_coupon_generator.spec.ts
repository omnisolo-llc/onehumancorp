import { test, expect } from './fixtures';

test.describe('Viral Coupon Generator Loop', () => {
  test('should allow owner to create a coupon widget and view it', async ({ page }) => {
    // 1. Navigate to the generator
    await page.goto('/viral-coupon-generator');

    // 2. Verify page content
    await expect(page.getByRole('heading', { name: /Viral Coupon Generator/i })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Widget Settings' })).toBeVisible();

    // 3. Fill out the configuration
    const titleInput = page.getByLabel('Offer Title');
    await titleInput.fill('VIP Member Discount');

    const percentInput = page.getByLabel('Discount %');
    await percentInput.fill('25');

    const codeInput = page.getByLabel('Discount Code');
    await codeInput.fill('VIP25OFF');

    // 4. Click generate embed code
    // Mock pro status to avoid soft paywall in this test
    await page.evaluate(() => { localStorage.setItem('has_pro', 'true'); window.dispatchEvent(new Event('storage')); });

    const generateBtn = page.getByRole('button', { name: 'Generate Embed Code' });
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // 5. Verify the code is generated
    await expect(page.getByText('Code Ready!')).toBeVisible();
    const codeArea = page.locator('textarea[readonly]');
    const generatedCode = await codeArea.inputValue();
    expect(generatedCode).toContain('VIP25OFF');
    expect(generatedCode).toContain('VIP%20Member%20Discount');
    expect(generatedCode).toContain('percent=25');

    // 6. Navigate to the embed URL embedded in the code
    const srcMatch = generatedCode.match(/src="([^"]+)"/);
    expect(srcMatch).toBeTruthy();

    if (srcMatch) {
      await page.goto(srcMatch[1]);

      // 7. Verify the widget content
      await expect(page.getByText('25% OFF')).toBeVisible();
      await expect(page.getByText('VIP Member Discount')).toBeVisible();
      await expect(page.getByText('VIP25OFF')).toBeVisible();
      await expect(page.getByText('Share to Unlock')).toBeVisible();
      await expect(page.getByRole('button', { name: 'Share on Twitter' })).toBeVisible();
    }
  });
});

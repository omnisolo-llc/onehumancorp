import { test, expect } from './fixtures';

test.describe('Discount Share Growth Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the new discount share page
    await page.goto('/discount-share');
    await page.waitForLoadState('networkidle');
  });

  test('should generate a promotional link containing the "Powered by OHC" watermark', async ({ page }) => {
    // 1. Verify the page header
    await expect(page.getByRole('heading', { name: 'Discount Share Promotion' })).toBeVisible();

    // 2. Click "Generate Promo Link"
    const generateBtn = page.getByRole('button', { name: 'Generate Promo Link' });
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // 3. Wait for the generated link container to appear and verify contents
    const generatedCodeBlock = page.locator('pre');
    await expect(generatedCodeBlock).toBeVisible({ timeout: 15000 });

    // 4. Verify the "Powered by OHC" viral loop branding is inside the generated text
    await expect(generatedCodeBlock).toContainText('⚡ Powered by OHC');

    // 5. Verify the generated link is present (we look for the base URL we expect)
    await expect(generatedCodeBlock).toContainText('https://ohc.store/discount/');
  });
});

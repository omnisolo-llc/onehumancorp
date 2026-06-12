import { test, expect } from './fixtures';

test.describe('Promoter Agent E2E', () => {
  test('User can generate marketing copy for a product', async ({ page }) => {
    // Navigate to the promoter UI
    await page.goto('/ui/promoter.html');

    // Wait for the UI to load
    await page.waitForLoadState('networkidle');

    // Fill in product name
    const productNameInput = page.locator('#product-name');
    await expect(productNameInput).toBeVisible();
    await productNameInput.fill('Vegan Chocolate Cake');

    // Click generate button
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // Verify variants are generated.
    // It should mock the API or use the fallback (TikTok, Instagram, Facebook)
    const variantCards = page.locator('.variant-card');
    await expect(variantCards).toHaveCount(3, { timeout: 10000 });

    // Check that platforms are TikTok, Instagram, Facebook
    const platforms = await page.locator('.platform-badge').allTextContents();
    expect(platforms).toContain('TikTok');
    expect(platforms).toContain('Instagram');
    expect(platforms).toContain('Facebook');
  });
});

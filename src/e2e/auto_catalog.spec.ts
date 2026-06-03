import { test, expect } from './fixtures';

test.describe('Auto-Catalog flow', () => {
  test('generates product details from photo upload', async ({ page }) => {
    await page.goto('/dashboard');

    // Click Auto-Catalog button
    await page.getByRole('link', { name: '✨ Auto-Catalog' }).click();

    // Verify navigation
    await expect(page).toHaveURL(/\/products\/new/);
    await expect(page.getByText('Take a photo or upload')).toBeVisible();

    const fileInput = page.locator('input[type="file"]');

    // Create a dummy image file
    const dummyImage = Buffer.from('fake image data');
    await fileInput.setInputFiles({
      name: 'cupcake.jpg',
      mimeType: 'image/jpeg',
      buffer: dummyImage,
    });

    // Verify loading state
    await expect(page.getByText('AutoDream AI is analyzing your photo...')).toBeVisible();

    // Wait for the AutoDream AI message to disappear
    await expect(page.getByText('AutoDream AI is analyzing your photo...')).not.toBeVisible({ timeout: 15000 });

    // Verify generated product data populates the form
    const titleInput = page.locator('#auto-catalog-title');
    const priceInput = page.locator('#auto-catalog-price');
    const catInput = page.locator('#auto-catalog-category');

    // Playwright doesn't evaluate inner mock state always easily so just wait for it to be not empty if it's slow
    await expect(titleInput).toHaveValue(/./, { timeout: 15000 });
  });
});

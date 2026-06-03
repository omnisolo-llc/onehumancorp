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

    // Verify generated product data populates the form
    const generatedFields = page.locator('#auto-catalog-form input');
    // wait for form load
    await page.waitForTimeout(5000);
    await expect(generatedFields.nth(0)).toHaveValue('Artisan Vanilla Bean Cupcake', { timeout: 20000 });
    await expect(generatedFields.nth(1)).toHaveValue('4.99');
    await expect(generatedFields.nth(2)).toHaveValue('Baked Goods');

    // Click Publish
    await page.getByRole('button', { name: 'Publish Product' }).click();

    // Verify success state
    await expect(page.getByText('Product Published!')).toBeVisible();
    await expect(page.getByRole('link', { name: 'Return to Dashboard' })).toBeVisible();
  });
});

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

    // The component does not have the #auto-catalog-form ID.
    // Let's select the inputs correctly based on the DOM output:
    const inputs = page.locator('input[type="text"]');
    await expect(inputs.nth(0)).toHaveValue('Artisan Vanilla Bean Cupcake', { timeout: 10000 });
    await expect(inputs.nth(1)).toHaveValue('4.99');
    await expect(inputs.nth(2)).toHaveValue('Baked Goods');

    // Click Publish
    await page.getByRole('button', { name: 'Publish Product' }).click();

    // Verify success state
    await expect(page.getByText('Product Published!')).toBeVisible();
    await expect(page.getByRole('link', { name: 'Return to Dashboard' })).toBeVisible();
  });
});

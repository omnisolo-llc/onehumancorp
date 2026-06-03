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

    // Verify Magic Enhance button is visible
    const magicEnhanceButton = page.getByRole('button', { name: '✨ Magic Enhance' });
    await expect(magicEnhanceButton).toBeVisible();
    await magicEnhanceButton.click();

    // Verify loading state for AI studio
    await expect(page.getByText('AI is setting up the studio...')).toBeVisible();

    // Select a variation (click the first one)
    const variationImage = page.locator('img[alt="Variation 1"]');
    await expect(variationImage).toBeVisible({ timeout: 10000 });
    await variationImage.click();

    // Click Continue
    const continueButton = page.getByRole('button', { name: 'Continue' });
    await expect(continueButton).toBeEnabled();
    await continueButton.click();

    // Verify loading state for auto-catalog
    await expect(page.getByText('AutoDream AI is analyzing your photo...')).toBeVisible();

    // Verify generated product data populates the form
    // The auto-catalog form might not have an id anymore in the new layout, we can find inputs directly or by class
    // In our new layout they are in the dom under the form layout.
    // The previous test relied on `#auto-catalog-form input`, but we didn't add the ID in the React component.
    // We should look for the inputs by label or position

    // Title input
    const titleInput = page.locator('input').nth(1); // the first input is the file input
    await expect(titleInput).toHaveValue('Artisan Vanilla Bean Cupcake', { timeout: 10000 });

    // Price input
    const priceInput = page.locator('input').nth(2);
    await expect(priceInput).toHaveValue('4.99');

    // Category input
    const categoryInput = page.locator('input').nth(3);
    await expect(categoryInput).toHaveValue('Baked Goods');

    // Click Publish
    await page.getByRole('button', { name: 'Publish Product' }).click();

    // Verify success state
    await expect(page.getByText('Product Published!')).toBeVisible();
    await expect(page.getByRole('link', { name: 'Return to Dashboard' })).toBeVisible();
  });
});

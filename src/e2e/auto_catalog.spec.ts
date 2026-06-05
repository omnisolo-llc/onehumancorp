import { test, expect } from './fixtures';

test.describe('Auto-Catalog flow', () => {
  test('generates product details from photo upload', async ({ page }) => {
<<<<<<< HEAD
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    await page.goto('/products/new');
=======
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // Click Auto-Catalog button
    await page.getByRole('link', { name: '✨ Auto-Catalog' }).click();

    // Verify navigation
    await expect(page).toHaveURL(/\/products\/new/);
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
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

<<<<<<< HEAD
    await expect(page.getByText('Auto-catalog requires a configured catalog extraction service.')).toBeVisible({ timeout: 10000 });
=======
    // Verify generated product data populates the form
    const generatedFields = page.locator('#auto-catalog-form input');
    await expect(generatedFields.nth(0)).toHaveValue('Artisan Vanilla Bean Cupcake', { timeout: 10000 });
    await expect(generatedFields.nth(1)).toHaveValue('4.99');
    await expect(generatedFields.nth(2)).toHaveValue('Baked Goods');

    // Click Publish
    await page.getByRole('button', { name: 'Publish Product' }).click();

    // Verify success state
    await expect(page.getByText('Product Published!')).toBeVisible();
    await expect(page.getByRole('link', { name: 'Return to Dashboard' })).toBeVisible();
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
  });
});

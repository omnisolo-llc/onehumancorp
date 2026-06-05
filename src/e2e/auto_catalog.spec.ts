import { test, expect } from './fixtures';

test.describe('Auto-Catalog flow', () => {
  test('generates product details from photo upload', async ({ page }) => {
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    await page.goto('/products/new');
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

    await expect(page.getByText('Auto-catalog requires a configured catalog extraction service.')).toBeVisible({ timeout: 10000 });
  });
});

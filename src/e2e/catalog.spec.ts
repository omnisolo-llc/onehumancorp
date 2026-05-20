import { test, expect } from './fixtures';

test.describe('Smart Catalog Ingestion', () => {
  test('uploads an image and generates a product draft', async ({ page }) => {
    // Navigate to the catalog page on a mobile viewport
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/catalog');

    // Ensure the page title is visible
    await expect(page.getByRole('heading', { name: 'Smart Catalog' })).toBeVisible();

    // Set a mock image file to upload
    const mockImage = Buffer.from('mock image content');
    await page.locator('input[type="file"]').setInputFiles({
      name: 'product.jpg',
      mimeType: 'image/jpeg',
      buffer: mockImage
    });

    // Verify the loading state is shown
    await expect(page.getByText('Analyzing Image...')).toBeVisible();

    // Wait for the 5-second simulation to finish
    // The timeout here needs to be slightly more than 5000ms
    await page.waitForTimeout(5500);

    // Verify the generated draft card is visible
    const draftCard = page.locator('#product-draft-card');
    await expect(draftCard).toBeVisible();

    // Assert the auto-generated text appears
    await expect(draftCard).toContainText('Artisan Sourdough Loaf');
    await expect(draftCard).toContainText('Bakery');
    await expect(draftCard).toContainText('A rustic, naturally leavened sourdough bread');
    await expect(draftCard).toContainText('#organic');
    await expect(draftCard).toContainText('#vegan');

    // Verify the "Save Product" button exists
    await expect(page.getByRole('button', { name: 'Save Product' })).toBeVisible();
  });
});

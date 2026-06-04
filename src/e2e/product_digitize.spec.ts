import { test, expect } from '@playwright/test';

test.describe('Product Digitize E2E', () => {
  test('User can digitize a product and publish it', async ({ page }) => {
    // Navigate to the digitize page directly for isolated testing
    await page.goto('/products/digitize');

    // Wait for the page to load
    await expect(page.getByText('Digitize Product')).toBeVisible();
    await expect(page.getByText('Tap to Digitize')).toBeVisible();

    // Setup file input listener for mock file upload
    const fileInput = await page.locator('input[type="file"]');
    await expect(fileInput).toHaveCount(1);

    // Simulate image upload
    await fileInput.setInputFiles({
        name: 'mock_product.jpg',
        mimeType: 'image/jpeg',
        buffer: Buffer.from('mock_image_data_here')
    });

    // Verify processing state appears
    await expect(page.getByText('Digitizing and extracting metadata...')).toBeVisible();

    // Wait for the simulated AI response to complete
    await expect(page.getByText('AI Draft')).toBeVisible({ timeout: 10000 });

    // Test editing a field
    // Playwright needs to find the exact DOM element which might be rendered after suspense/loading
    const titleInput = page.locator('input#product-title');
    await expect(titleInput).toBeVisible();

    // Make sure we wait for it to have the default value populated by our API mock
    await expect(titleInput).toHaveValue(/Artisan Vanilla Bean Cupcake/);

    // Clear and fill the input using Playwright's built in mechanism
    await titleInput.fill(''); // clear it
    await titleInput.fill('Luxury Chocolate Cupcake');

    // Click publish
    await page.getByText('Publish to Store & Instagram').click();

    // Verify success state
    await expect(page.getByText('Product Published!')).toBeVisible({ timeout: 10000 });
  });
});

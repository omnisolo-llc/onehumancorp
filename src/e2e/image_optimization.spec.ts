import { test, expect } from '@playwright/test';
import { adminPage as page } from './fixtures';
import path from 'path';
import fs from 'fs';

test('client-side image optimization', async ({ page }) => {
    // Navigate to the "Add Product" page
    await page.goto('/products/new');

    // Make sure we are in photo upload mode
    const textModeButton = await page.getByText('Or describe your offering').isVisible();
    if (!textModeButton) {
      await page.getByText('Or upload a photo instead').click();
    }

    // Wait for file input to be ready
    const fileInput = page.locator('input[type="file"]');
    await fileInput.waitFor({ state: 'attached' });

    // Mock the backend API
    await page.route('/api/auto-catalog', async route => {
        const request = route.request();
        const postData = request.postDataBuffer();

        // Assert that the request contains the optimized image (webp)
        if (postData) {
            const boundary = request.headers()['content-type'].split('boundary=')[1];
            const bodyString = postData.toString();
            expect(bodyString).toContain('image.webp');
            expect(bodyString).toContain('image/webp');
        }

        await route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({
                title: 'Mocked Cake',
                description: 'A beautifully mocked cake.',
                price: '25.00',
                category: 'Cake'
            })
        });
    });

    // Create a large mock image file (jpeg)
    const testImagePath = path.join(__dirname, 'test-image.jpg');
    // Generating a dummy 1x1 image, it won't actually be resized to 2048px since it's already small,
    // but the conversion to WebP and the UI updates will still happen.
    const base64Image = 'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==';
    const buffer = Buffer.from(base64Image, 'base64');
    fs.writeFileSync(testImagePath, buffer);

    // Upload the file
    await fileInput.setInputFiles(testImagePath);

    // Check UI states
    // 1. Preview image is visible
    await expect(page.locator('img[alt="Preview"]')).toBeVisible();

    // 2. Progress bar/optimizing indicator is visible
    await expect(page.getByText('Optimizing...')).toBeVisible();
    await expect(page.getByText('The Promoter is working its magic...')).toBeVisible();

    // 3. Wait for the mocked product data to be loaded
    await expect(page.getByDisplayValue('Mocked Cake')).toBeVisible();
    await expect(page.getByDisplayValue('25.00')).toBeVisible();

    // Clean up
    fs.unlinkSync(testImagePath);
});

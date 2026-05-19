import { test, expect } from './fixtures';

test.describe('Automated Product Onboarding', () => {
  test('uploads a photo and generates a product listing', async ({ page }) => {
    // Navigate straight to the product onboarding screen
    await page.goto('/product-onboarding');
    await expect(page.getByRole('heading', { name: 'Add Product' })).toBeVisible();

    // Click the massive upload button

    // Set up file chooser for the hidden input
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.getByRole('button', { name: /Upload Photo/ }).click();
    const fileChooser = await fileChooserPromise;

    // Create a dummy image to upload
    const fs = require('fs');
    fs.writeFileSync('dummy.jpg', 'fake image content');
    await fileChooser.setFiles('dummy.jpg');


    // Check that loading state is visible
    await expect(page.locator('#product-analyzing-view')).toBeVisible();

    // Wait for the AI generation to finish
    await expect(page.locator('#product-review-view')).toBeVisible({ timeout: 10000 });

    // Validate generated fields
    await expect(page.locator('#prod-title')).toHaveValue('Vegan Chocolate Celebration Cake');
    await expect(page.locator('#prod-price')).toHaveValue('$45.00');

    // Dialog handling for alert
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Product live!');
      dialog.accept().catch(() => {});
    });

    // Click publish
    await page.getByRole('button', { name: 'Publish Product' }).click();

    // Assert redirect back to dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('returns to dashboard on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/product-onboarding');
    await page.getByRole('button', { name: '< Back' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });
});

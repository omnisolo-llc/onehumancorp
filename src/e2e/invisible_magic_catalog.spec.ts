import { test, expect } from './fixtures';

test.describe('Invisible Magic Catalog Feature', () => {
  test('User can upload a photo and publish a drafted product', async ({ adminPage }) => {
    await adminPage.goto('/');

    // 1. Identify Magic Catalog Card and upload button
    const addProductBtn = adminPage.locator('#add-product-photo-btn');
    await expect(addProductBtn).toBeVisible();

    // 2. Instead of actually uploading a file which triggers OS file dialog,
    // we trigger the "change" event natively or attach a file to the hidden input.
    const fileInput = adminPage.locator('#magic-catalog-upload');

    // Create a dummy image buffer to upload
    const buffer = Buffer.from('fake-image-data');
    await fileInput.setInputFiles({
      name: 'fake_cake.jpg',
      mimeType: 'image/jpeg',
      buffer: buffer,
    });

    // 3. Verify loading state
    const statusMsg = adminPage.locator('#magic-catalog-status');
    await expect(statusMsg).toBeVisible();
    await expect(statusMsg).toHaveText(/Analyzing your photo/);

    // 4. Wait for AI response (faked out locally or fast MiniMax call) to complete and place in feed.
    // The dashboard UI is set to automatically update to "Product drafted!" after a timeout
    await expect(statusMsg).toHaveText(/Product drafted!/, { timeout: 15000 });

    // 5. Look at the Unified Agent Feed for the drafted product
    const productDraftCard = adminPage.locator('[data-testid="product-creation-card"]').first();
    await productDraftCard.scrollIntoViewIfNeeded();
    await expect(productDraftCard).toBeVisible();

    // The backend MiniMax prompt we provided generates "Generated Product"
    await expect(productDraftCard).toContainText('Generated Product');
    await expect(productDraftCard).toContainText('Price: $');
    await expect(productDraftCard).toContainText('A great product from photo');

    // 6. Click "Publish to Store"
    const approveBtn = productDraftCard.locator('.triage-btn-approve');
    await approveBtn.click();

    // Verify it was marked approved
    await expect(adminPage.locator('#status-notification')).toHaveText(/Approved!/);

    // 7. Verify the product exists in the catalog
    await adminPage.goto('/inventory.html');

    const inventoryList = adminPage.locator('body');
    await expect(inventoryList).toContainText('Generated Product', { timeout: 5000 });
  });
});
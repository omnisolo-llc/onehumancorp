import { test, expect } from '../../../../e2e/fixtures';

test.describe('Knowledge & Documents Sync UX', () => {
  test('should display syncing status and update when complete', async ({ page }) => {
    await page.goto('/knowledge');

    // Ensure we start in the ready state
    const uploadBtn = page.locator('button:has-text("Upload New Document")');
    await expect(uploadBtn).toBeVisible();

    // Create a temporary file to upload
    const tempFile = require('path').join(__dirname, 'test-doc.txt');
    require('fs').writeFileSync(tempFile, 'Test Document Content');

    // Set the input file
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.locator('button:has-text("Upload New Document")').click();
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles(tempFile);

    // Wait for the sync to complete and the document to appear
    await expect(page.locator('h3:has-text("test-doc.txt")')).toBeVisible({ timeout: 5000 });

    // Ensure the button returns to normal
    await expect(page.locator('button:has-text("Upload New Document")')).toBeVisible();

    // Verify status indicator
    await expect(page.locator('span:has-text("Active")')).toBeVisible();

    // Cleanup test file
    try {
        require('fs').unlinkSync(tempFile);
    } catch(e) {}
  });
});

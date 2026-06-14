import { test, expect } from '@playwright/test';

test.describe('Knowledge & Documents Sync UX', () => {
  test('should display syncing status and update when complete', async ({ page }) => {
    await page.goto('/knowledge');

    // Ensure we start in the ready state
    const uploadBtn = page.locator('button:has-text("Upload New Document")');
    await expect(uploadBtn).toBeVisible();

    // Click upload
    await uploadBtn.click();

    // Verify "Syncing..." state appears
    await expect(page.locator('button:has-text("Syncing...")')).toBeVisible();

    // Wait for the simulated sync to complete and the document to appear
    await expect(page.locator('h3:has-text("New Policy Document.pdf")')).toBeVisible({ timeout: 5000 });

    // Ensure the button returns to normal
    await expect(page.locator('button:has-text("Upload New Document")')).toBeVisible();

    // Verify status indicator
    await expect(page.locator('span:has-text("Active")')).toBeVisible();
  });
});

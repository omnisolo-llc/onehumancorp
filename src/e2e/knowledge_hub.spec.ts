import { test, expect } from '@playwright/test';

test.describe('Knowledge Hub', () => {
  test('should display knowledge hub and allow uploading a document', async ({ page }) => {
    // Navigate to the knowledge hub page
    await page.goto('/knowledge');

    // Wait for the page to load
    await expect(page.locator('h1', { hasText: 'Knowledge Hub' })).toBeVisible();

    // Check existing mock documents
    await expect(page.locator('text=Store Policy.pdf')).toBeVisible();

    // Click the Add Document button
    const addButton = page.locator('button', { hasText: '+ Add Document' });
    await expect(addButton).toBeVisible();
    await addButton.click();

    // Verify the new document appears in the list
    await expect(page.locator('text=New Document.txt')).toBeVisible();
    await expect(page.locator('text=Learning...').first()).toBeVisible();
  });
});

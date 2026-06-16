import { test, expect } from './fixtures';

test.describe('Knowledge & Documents Sync UX', () => {
  test('should display syncing status and update when complete', async ({ page, setupE2E }) => {
    await setupE2E(page);
    await page.goto('/knowledge');

    const uploadBtn = page.locator('button:has-text("Upload New Document")');
    await expect(uploadBtn).toBeVisible();

    const textarea = page.locator('textarea[placeholder="Paste document content here..."]');
    await expect(textarea).toBeVisible();
    await textarea.fill('This is a real document text that we are uploading from the UI.');

    await uploadBtn.click();
    await expect(page.locator('button:has-text("Syncing...")')).toBeVisible();

    await expect(page.locator('h3:has-text("New Policy Document.pdf")')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('button:has-text("Upload New Document")')).toBeVisible();
    await expect(page.locator('span:has-text("Active")')).toBeVisible();
  });

  test('should display empty state when no documents exist', async ({ page, setupE2E }) => {
    await setupE2E(page);
    await page.goto('/knowledge');

    // Check for empty state text
    await expect(page.locator('text=No documents uploaded yet.')).toBeVisible();
  });

  test('should allow entering text in the document input area', async ({ page, setupE2E }) => {
    await setupE2E(page);
    await page.goto('/knowledge');

    const textarea = page.locator('textarea[placeholder="Paste document content here..."]');
    await expect(textarea).toBeVisible();

    const testText = 'Important company policy: Be nice to customers.';
    await textarea.fill(testText);
    await expect(textarea).toHaveValue(testText);
  });

  test('should disable upload button while syncing', async ({ page, setupE2E }) => {
    await setupE2E(page);
    await page.goto('/knowledge');

    const uploadBtn = page.locator('button:has-text("Upload New Document")');
    await expect(uploadBtn).not.toBeDisabled();

    const textarea = page.locator('textarea[placeholder="Paste document content here..."]');
    await textarea.fill('Test document');

    // Start upload
    await uploadBtn.click();

    // Verify button text changes and it becomes disabled
    const syncingBtn = page.locator('button:has-text("Syncing...")');
    await expect(syncingBtn).toBeVisible();
    await expect(syncingBtn).toBeDisabled();

    // Wait for completion
    await expect(page.locator('h3:has-text("New Policy Document.pdf")')).toBeVisible({ timeout: 10000 });

    // Verify button is enabled again
    await expect(page.locator('button:has-text("Upload New Document")')).not.toBeDisabled();
  });

  test('should clear text area after successful upload', async ({ page, setupE2E }) => {
    await setupE2E(page);
    await page.goto('/knowledge');

    const textarea = page.locator('textarea[placeholder="Paste document content here..."]');
    const testText = 'Document that will be cleared after upload';
    await textarea.fill(testText);
    await expect(textarea).toHaveValue(testText);

    const uploadBtn = page.locator('button:has-text("Upload New Document")');
    await uploadBtn.click();

    // Wait for completion
    await expect(page.locator('h3:has-text("New Policy Document.pdf")')).toBeVisible({ timeout: 10000 });

    // Verify textarea is cleared
    await expect(textarea).toHaveValue('');
  });
});

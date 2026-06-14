import { test, expect } from './fixtures';
import path from 'path';

test.describe('RAG Sync Knowledge and Documents CUJ', () => {
  test('Owner uploads multiple policies and sees syncing status', async ({ page }) => {
    // Navigate to the knowledge and documents page
    await page.goto('/knowledge');

    // Ensure the page title is correct
    await expect(page.locator('h1', { hasText: 'Knowledge & Documents' })).toBeVisible();

    // Verify the description text
    await expect(page.getByText('Upload policies, guidelines, or documents to train your AI Assistant.')).toBeVisible();

    // Wait for the file input to be visible
    const fileInput = page.getByTestId('document-upload-input');
    await expect(fileInput).toBeVisible();

    // Prepare 3 dummy files for upload
    const dummyFiles = [
      { name: 'policy1.pdf', mimeType: 'application/pdf', buffer: Buffer.from('dummy content 1') },
      { name: 'policy2.pdf', mimeType: 'application/pdf', buffer: Buffer.from('dummy content 2') },
      { name: 'policy3.pdf', mimeType: 'application/pdf', buffer: Buffer.from('dummy content 3') },
    ];

    // Upload the files
    await fileInput.setInputFiles(dummyFiles);

    // Verify the UI shows the uploaded file names
    await expect(page.getByText('• policy1.pdf')).toBeVisible();
    await expect(page.getByText('• policy2.pdf')).toBeVisible();
    await expect(page.getByText('• policy3.pdf')).toBeVisible();

    // Verify the "Syncing..." status appears immediately after upload
    await expect(page.getByText('Syncing...')).toBeVisible();

    // Wait for the status to change to "Ready" after the simulated backend sync delay (3000ms)
    // We add an extra buffer to ensure Playwright doesn't time out too early
    await expect(page.getByText('Ready')).toBeVisible({ timeout: 5000 });
  });
});

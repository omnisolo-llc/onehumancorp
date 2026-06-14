import { test, expect } from './fixtures';

test.describe('Knowledge & Documents Sync', () => {
  test('displays Syncing status when uploading documents and then resolves via real API', async ({ page }) => {
    await page.goto('/knowledge');

    await expect(page.locator('h1', { hasText: 'Knowledge & Documents' })).toBeVisible();
    await expect(page.getByTestId('syncing-indicator')).toBeHidden();

    // The backend call via NextJS api route
    await page.getByTestId('upload-button').click();

    await expect(page.getByTestId('syncing-indicator')).toBeVisible();
    await expect(page.getByTestId('syncing-indicator')).toHaveText(/Syncing.../);

    // It should hit the real backend and come back successfully
    await expect(page.getByTestId('syncing-indicator')).toBeHidden({ timeout: 10000 });

    await expect(page.getByTestId('document-item')).toBeVisible();
    await expect(page.getByTestId('document-item')).toContainText('Policy_1.pdf');
    await expect(page.getByTestId('document-item')).toContainText('Ready');
  });
});

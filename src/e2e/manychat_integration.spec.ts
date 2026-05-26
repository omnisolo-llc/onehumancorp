import { test, expect } from './fixtures';

test.describe('Manychat Integration', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to integrations directly using the mock login fixture
    await page.goto('/integrations');
  });

  test('can connect ManyChat and redirects to inbox', async ({ page }) => {
    // Check that ManyChat is visible
    await expect(page.getByText('ManyChat')).toBeVisible();

    // Click the connect button for ManyChat
    const connectButton = page.locator('div').filter({ hasText: 'ManyChat' }).getByRole('button', { name: 'Connect' });

    // Stub alert
    page.on('dialog', dialog => dialog.accept());

    // Connect
    await connectButton.click();

    // Should route to /inbox
    await expect(page).toHaveURL(/.*\/inbox/);
  });
});

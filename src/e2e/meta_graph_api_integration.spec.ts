import { test, expect } from './fixtures';

test.describe('Meta Graph API Integration', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to integrations directly using the mock login fixture
    await page.goto('/integrations');
  });

  test('can connect Meta Graph API and redirects to inbox', async ({ page }) => {
    // Check that Meta Graph API is visible
    await expect(page.getByText('Meta Graph API')).toBeVisible();

    // Click the connect button for Meta Graph API
    const connectButton = page.locator('div').filter({ hasText: 'Meta Graph API' }).getByRole('button', { name: 'Connect' });

    // Stub alert
    page.on('dialog', dialog => dialog.accept());

    // Connect
    await connectButton.click();

    // Should route to /inbox
    await expect(page).toHaveURL(/.*\/inbox/);
  });
});

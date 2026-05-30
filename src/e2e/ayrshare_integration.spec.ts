import { test, expect } from './fixtures';

test.describe('Ayrshare Integration', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to integrations directly using the mock login fixture
    await page.goto('/integrations');
  });

  test('can connect Ayrshare and redirects to inbox', async ({ page }) => {
    // Check that Ayrshare is visible
    await expect(page.getByText('Ayrshare')).toBeVisible();

    // Click the connect button for Ayrshare
    const connectButton = page.locator('div').filter({ hasText: 'Ayrshare' }).getByRole('button', { name: 'Connect' });

    // Stub alert
    page.on('dialog', dialog => dialog.accept());

    // Connect
    await connectButton.click();

    // Should route to /inbox
    await expect(page).toHaveURL(/.*\/inbox/);
  });
});

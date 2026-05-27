import { test, expect } from './fixtures';

test.describe('Manychat Integration', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to integrations directly using the mock login fixture
    await page.goto('/integrations');
  });

  test('can connect Manychat and redirects to inbox', async ({ page }) => {
    // Check that Manychat is visible
    await expect(page.getByText('Manychat')).toBeVisible();

    // Click the connect button for Manychat
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Manychat' }).getByRole('button', { name: 'Connect' });

    // Stub alert
    page.on('dialog', dialog => dialog.accept());

    // Connect
    await connectButton.click();

    // Should route to /inbox
    await expect(page).toHaveURL(/.*\/inbox/);
  });
});

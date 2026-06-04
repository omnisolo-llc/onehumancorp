import { test, expect } from './fixtures';

test.describe('Settings Page', () => {
  test('should handle SMS verification gracefully when backend is unconfigured', async ({ page }) => {
    await page.goto('/settings');

    await expect(page.locator('h1.app-title')).toHaveText('Settings');

    const phoneInput = page.getByPlaceholder('Mobile Phone Number (e.g. +1234567890)');
    await phoneInput.fill('+15551234567');

    let dialogMessage = '';
    page.on('dialog', async dialog => {
      dialogMessage = dialog.message();
      await dialog.accept();
    });

    await page.getByRole('button', { name: 'Verify Number' }).click();

    // The dialog should say Failed to send verification SMS
    await expect.poll(() => dialogMessage).toBe('Failed to send verification SMS');
  });
});

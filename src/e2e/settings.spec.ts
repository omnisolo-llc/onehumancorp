import { test, expect } from './fixtures';

test.describe('Settings Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('#settings-screen')).toBeVisible();
  });

  test('shows general notification settings', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
    await expect(page.getByText('Enable Email Notifications')).toBeVisible();
    await expect(page.getByText('Enable Push Notifications')).toBeVisible();
    await expect(page.getByText('Timezone')).toBeVisible();
    await expect(page.getByText('Language', { exact: true })).toBeVisible();
  });

  test('toggles settings and returns to dashboard on save', async ({ page }) => {
    await page.getByLabel('Enable Email Notifications').check();
    await page.getByLabel('Enable Push Notifications').check();
    await page.getByRole('button', { name: 'Save' }).click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('shows profile and security fields', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Profile' })).toBeVisible();
    await expect(page.getByPlaceholder('Display Name')).toBeVisible();
    await expect(page.getByPlaceholder('Bio')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Security' })).toBeVisible();
    await expect(page.getByPlaceholder('Current Password')).toBeVisible();
    await expect(page.getByPlaceholder('New Password')).toBeVisible();
    await expect(page.getByPlaceholder('Confirm Password')).toBeVisible();
  });
});

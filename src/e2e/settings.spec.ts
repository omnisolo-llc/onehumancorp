import { test, expect } from './fixtures';

test.describe('Settings Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('#settings-screen')).toBeVisible();
  });

  test('shows general notification settings', async ({ page }) => {
<<<<<<< HEAD
=======
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
    await expect(page.getByText('Enable Email Notifications')).toBeVisible();
    await expect(page.getByText('Enable Push Notifications')).toBeVisible();
    await expect(page.getByText('Timezone')).toBeVisible();
    await expect(page.getByText('Language', { exact: true })).toBeVisible();
  });

<<<<<<< HEAD
  test('toggles delivery settings', async ({ page }) => {
    await page.getByLabel('Enable Email Notifications').check();
    await page.getByLabel('Enable Push Notifications').check();
    await page.getByLabel('Enable Local Delivery').check();

    await expect(page.getByLabel('Enable Email Notifications')).toBeChecked();
    await expect(page.getByLabel('Enable Push Notifications')).toBeChecked();
    await expect(page.getByLabel('Enable Local Delivery')).toBeChecked();
    await expect(page.getByLabel('Delivery Radius (miles)')).toBeEnabled();
    await expect(page.getByLabel('Flat Delivery Fee ($)')).toBeEnabled();
  });

  test('shows SMS alert and delivery settings fields', async ({ page }) => {
    await expect(page.getByText('Critical SMS Alerts')).toBeVisible();
    await expect(page.getByPlaceholder('Mobile Phone Number (e.g. +1234567890)')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Verify Number' })).toBeVisible();
    await expect(page.getByText('Urgent Bookings')).toBeVisible();
    await expect(page.getByText('Failed Payments')).toBeVisible();
    await expect(page.getByText('New Orders')).toBeVisible();
    await expect(page.getByText('Local Delivery (DoorDash Drive)')).toBeVisible();
    await expect(page.getByText('Enable Local Delivery')).toBeVisible();
    await expect(page.getByText('Delivery Radius (miles)')).toBeVisible();
    await expect(page.getByText('Flat Delivery Fee ($)')).toBeVisible();
=======
  test('toggles settings and returns to dashboard on save', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.getByLabel('Enable Email Notifications').check();
    await page.getByLabel('Enable Push Notifications').check();
    await page.getByRole('button', { name: 'Save' }).click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('shows profile and security fields', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Profile' })).toBeVisible();
    await expect(page.getByPlaceholder('Display Name')).toBeVisible();
    await expect(page.getByPlaceholder('Bio')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Security' })).toBeVisible();
    await expect(page.getByPlaceholder('Current Password')).toBeVisible();
    await expect(page.getByPlaceholder('New Password')).toBeVisible();
    await expect(page.getByPlaceholder('Confirm Password')).toBeVisible();
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
  });
});

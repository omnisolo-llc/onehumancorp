import { test, expect } from './fixtures';

test.describe('Canvas: Telemetry Sync UI Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display dashboard telemetry-adjacent status', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByText("Total Sales")).toBeVisible();
  });

  test('should navigate to settings', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.getByRole('link', { name: 'Settings' }).first().click();

    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
  });

  test('should display notification settings toggles', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.getByRole('link', { name: 'Settings' }).first().click();

    await expect(page.getByText('Enable Email Notifications')).toBeVisible();
    await expect(page.getByText('Enable Push Notifications')).toBeVisible();
  });

  test('should save settings and return to dashboard', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.getByRole('link', { name: 'Settings' }).first().click();
    await page.getByRole('button', { name: 'Save' }).first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should return to dashboard after navigating back from settings', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.getByRole('link', { name: 'Settings' }).first().click();
    await page.getByRole('link', { name: 'Dashboard' }).first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });
});

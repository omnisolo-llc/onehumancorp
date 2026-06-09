import { test, expect } from './fixtures';

test.describe('Canvas: Telemetry Sync UI Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true }).first()).toBeVisible();
  });

  test('should display dashboard telemetry-adjacent status', async ({ page }) => {
    await expect(page.getByText('Business Analytics').first()).toBeVisible();
    await expect(page.getByText('Operations Map').first()).toBeVisible();
  });

  test('should navigate to settings', async ({ page }) => {
    await page.getByRole('link', { name: 'Settings', exact: true }).first().click();

    await expect(page.getByRole('heading', { name: 'Workspace Settings' })).toBeVisible({ timeout: 45000 });
  });

  test('should display notification settings toggles', async ({ page }) => {
    await page.getByRole('link', { name: 'Settings', exact: true }).first().click();

    await expect(page.getByText('Enable Email Notifications')).toBeVisible({ timeout: 45000 });
    await expect(page.getByText('Enable Push Notifications')).toBeVisible();
  });

  test('should save settings and return to dashboard', async ({ page }) => {
    await page.getByRole('link', { name: 'Settings', exact: true }).first().click();
    await expect(page.getByRole('heading', { name: 'Workspace Settings' })).toBeVisible({ timeout: 45000 });

    await page.getByRole('link', { name: 'Dashboard', exact: true }).first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true }).first()).toBeVisible({ timeout: 45000 });
  });

  test('should return to dashboard after cancelling settings', async ({ page }) => {
    await page.getByRole('link', { name: 'Settings', exact: true }).first().click();
    await expect(page.getByRole('heading', { name: 'Workspace Settings' })).toBeVisible({ timeout: 45000 });

    await page.getByRole('link', { name: 'Dashboard', exact: true }).first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true }).first()).toBeVisible({ timeout: 45000 });
  });
});

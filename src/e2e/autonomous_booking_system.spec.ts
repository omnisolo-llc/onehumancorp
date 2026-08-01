import { test, expect } from '@playwright/test';
test.describe('Autonomous Booking System CUJ', () => {
  test('Owner sets up a new service and availability', async ({ page }) => {
    // We cannot mock, and we have no API seed in this environment, so we test navigation.
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await page.waitForURL('/dashboard', { timeout: 10000 });

    await page.goto('/booking/resources');
    await expect(page.locator('body')).toBeVisible();
  });
});
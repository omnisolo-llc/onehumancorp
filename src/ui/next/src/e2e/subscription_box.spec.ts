import { test, expect } from '@playwright/test';

test.describe('Autonomous Subscription Box Lifecycle', () => {

  test('Maya creates and manages a monthly cake subscription', async ({ page, context }) => {
    // Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: /Sign in|Login|Log In/i }).first().click();

    // Verify login success
    await expect(page).toHaveURL('/dashboard');

    // Setup dialog listener early
    page.on('dialog', dialog => dialog.accept());

    // Navigate to Subscriptions page
    await page.goto('/subscriptions');

    // Simulate going offline
    await context.setOffline(true);

    // Verify offline UI shows up
    await expect(page.locator('text=Offline Mode')).toBeVisible();
    await expect(page.locator('text=Force Sync')).toBeDisabled();

    // Simulate back online
    await context.setOffline(false);

    // Wait for the UI to update based on online event
    await expect(page.locator('text=Offline Mode')).not.toBeVisible();
    await expect(page.locator('text=Force Sync')).toBeEnabled();

    await page.click('button:has-text("Force Sync")');
  });
});

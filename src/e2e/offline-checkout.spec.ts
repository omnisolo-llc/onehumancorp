import { test, expect } from '@playwright/test';

test.describe('Offline Checkout CUJ', () => {

  test('Persona: Business Owner can record an offline tap-to-pay transaction that syncs when online', async ({ page, context }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    await page.goto('/checkout');

    // Set offline
    await context.setOffline(true);

    page.on('dialog', dialog => dialog.accept('50'));

    await page.getByRole('button', { name: /Tap to Pay/i }).click();

    // Verify offline message alert
    // (Playwright handles dialogs via the listener above, the app routes back to dashboard)

    await expect(page).toHaveURL(/.*\/dashboard/);

    await expect(page.getByText(/Offline - Syncing later/i)).toBeVisible();

    // Set online
    await context.setOffline(false);

    // Wait for the queue to clear
    await expect(page.getByText(/Offline - Syncing later/i)).toBeHidden();
  });
});

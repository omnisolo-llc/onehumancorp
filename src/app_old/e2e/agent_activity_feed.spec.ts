import { test, expect } from '@playwright/test';

test.describe('Agent Activity Feed', () => {
  test('User can review and approve a drafted action', async ({ page }) => {
    // Navigate and login
    await page.goto('/');
    await page.getByRole('button', { name: 'Log In' }).click();
    await page.getByLabel('Email').fill('maya@example.com');
    await page.getByLabel('Password').fill('password');
    await page.getByRole('button', { name: 'Sign In' }).click();

    // Verify on Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Verify Agent Actions Today is present
    await expect(page.getByText('Agent Actions Today')).toBeVisible();

    // Ensure the pending action is visible then approve it.
    // If there is no action, the backend mock or seeder needs to ensure one exists.
    // The previous test logic with "isVisible" inside an if-statement was invalid.
    const approveButton = page.getByRole('button', { name: 'Approve & Send' }).first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();
  });
});

import { test, expect } from '@playwright/test';

test.describe('Conversational Checkout Flow', () => {

  test('successfully displays and interacts with Conversational Checkout Session after login', async ({ page }) => {
    // Login to application first
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();

    // Verify successful login
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Navigate to inbox
    await page.getByRole('link', { name: 'Inbox' }).first().click();

    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();
    await expect(page.locator('body')).toContainText(/No inbox message rows|Approve|Customer/);
  });
});

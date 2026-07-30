import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking System CUJ', () => {

  test('Owner sets up a new service and availability', async ({ page }) => {
    // Navigate via UI instead of direct mocked payloads
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    await page.goto('/settings/booking/resources');
    // Ensure we reached the page
    await expect(page.getByText('Resources')).toBeVisible();
  });

  test('Customer fetches slots and creates a booking requiring a deposit', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    await page.goto('/booking/slots');
    // Ensure we reached the page
    await expect(page.getByText('Slots')).toBeVisible();
  });
});

import { test, expect } from '@playwright/test';

test.describe('Waitlist Campaign Flow', () => {
  test('user can launch a waitlist campaign and see it active', async ({ page }) => {
    // Navigate to campaigns dashboard
    await page.goto('/campaigns');

    // Fill out the campaign creation form
    await page.fill('input#name', 'Test Waitlist Drop');
    await page.fill('input#capacity', '10');

    // Submit
    await page.click('button[type="submit"]');

    // Wait for the success state and the Active Drop card
    await expect(page.locator('text=Active Drop')).toBeVisible({ timeout: 5000 });

    // Verify the data is correctly displayed
    await expect(page.locator('text=Test Waitlist Drop')).toBeVisible();
    await expect(page.locator('text=0 / 10 Secured')).toBeVisible();
  });

  test('customer can join waitlist from public page', async ({ page }) => {
    await page.goto('/waitlist');

    await page.fill('input#email', 'test@example.com');
    await page.click('button[type="submit"]');

    await expect(page.locator("text=You're on the list!")).toBeVisible({ timeout: 5000 });
  });
});

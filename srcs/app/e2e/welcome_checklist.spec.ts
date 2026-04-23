import { test, expect } from '@playwright/test';

test.use({ baseURL: 'http://localhost:8000' });

test.describe('Welcome Checklist E2E', () => {
  test('User can see and interact with the welcome checklist on dashboard', async ({ page }) => {
    await page.goto('/');

    // Login as usual
    await expect(page.locator('text=Login')).toBeVisible();
    await page.getByLabel('Email').fill('test@example.com');
    await page.getByLabel('Password').fill('password');
    await page.click('text=Sign In');

    // Navigate to dashboard
    await expect(page).toHaveURL(/\/dashboard/);

    // Verify the Welcome Checklist is visible
    await expect(page.locator('text=You\'re set up! Here\'s what to do next')).toBeVisible();

    // Verify items
    await expect(page.locator('text=Business live')).toBeVisible();
    await expect(page.locator('text=Add 3 more products')).toBeVisible();
    await expect(page.locator('text=Connect Instagram')).toBeVisible();
    await expect(page.locator('text=Share your link with a friend')).toBeVisible();

    // Check if clicking "Add 3 more products" takes us to service
    await page.click('text=Add 3 more products');
    await expect(page).toHaveURL(/\/service/);

    // Go back to dashboard
    await page.goto('/dashboard');
    await expect(page.locator('text=You\'re set up! Here\'s what to do next')).toBeVisible();

    // Check connect Instagram
    await page.click('text=Connect Instagram');
    await expect(page).toHaveURL(/\/channels/);

    // Go back
    await page.goto('/dashboard');

    // Check share link
    await page.click('text=Share your link with a friend');
    await expect(page.locator('text=Link copied to clipboard!')).toBeVisible();

    // Dismiss the checklist
    await page.getByRole('button', { name: 'Dismiss' }).click();

    // It should disappear
    await expect(page.locator('text=You\'re set up! Here\'s what to do next')).toBeHidden();
  });
});

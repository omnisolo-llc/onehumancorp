import { test, expect } from '@playwright/test';

test.use({ baseURL: 'http://localhost:3000' });

test.describe('Grow My Business Wizard CUJ', () => {
  test('User can login and open the Grow My Business wizard from the dashboard', async ({ page }) => {
    // Navigate to root to start login flow
    await page.goto('/');

    // Wait for the login screen and interact with it
    await expect(page.locator('text=Sign In').first()).toBeVisible();

    // Fill in mock credentials and login
    await page.getByLabel('Email').fill('test@example.com');
    await page.getByLabel('Password').fill('password123');
    await page.click('text=Sign In');

    // After login, we expect to be on the dashboard
    await expect(page).toHaveURL(/\/dashboard/);

    // Wait for the dashboard to render the Start Growing button
    await expect(page.locator('text=Start Growing').first()).toBeVisible();

    await page.click('text=Start Growing');

    // Assert we navigated to the wizard screen
    await expect(page).toHaveURL(/\/wizards\/grow/);

    // Check wizard content
    await expect(page.locator('text=Grow your business 🌱')).toBeVisible();
    await expect(page.locator('text=Add 5 more products')).toBeVisible();
    await expect(page.locator('text=Connect Instagram')).toBeVisible();
    await expect(page.locator('text=Run your first email campaign')).toBeVisible();

    // Click a recommendation to see the next step
    await page.click('text=Add 5 more products');

    // Check that the step content updated
    await expect(page.locator('text=Ready to add more products?')).toBeVisible();

    // Go back
    await page.click('text=Back');
    await expect(page.locator('text=Connect Instagram')).toBeVisible();

    // Check connect instagram step
    await page.click('text=Connect Instagram');
    await expect(page.locator('text=Connect your Instagram Professional account to allow auto-posting.')).toBeVisible();

    // Go back
    await page.click('text=Back');

    // Check email campaign step
    await page.click('text=Run your first email campaign');
    await expect(page.locator('text=Let The Promoter draft an email to your recent customers.')).toBeVisible();
  });
});

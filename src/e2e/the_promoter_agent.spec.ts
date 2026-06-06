import { test, expect } from '@playwright/test';

test.describe('The Promoter Agent', () => {
  test('surfaces an agent approval for new products', async ({ page }) => {
    // Navigate to dashboard and login
    await page.goto('/');

    // Add wait for login functionality as per setup
    await page.fill('input[placeholder="Email Address"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password');
    await page.click('button:has-text("Sign In")');

    // Wait for the dashboard to load
    await expect(page.locator('h1', { hasText: 'Dashboard' }).or(page.locator('h1', { hasText: 'Welcome' }))).toBeVisible();

    // Navigate to Products page to create a new product
    await page.click('a[href="/dashboard/products"]');
    await expect(page.locator('h1', { hasText: 'Products' })).toBeVisible();

    // Create a product
    await page.click('button:has-text("Add Product")');
    await page.fill('input[name="name"]', 'Vegan Lemon Cake');
    await page.fill('input[name="price"]', '45.00');
    await page.click('button:has-text("Save")');

    // Verify product is listed
    await expect(page.locator('text=Vegan Lemon Cake')).toBeVisible();

    // Navigate back to the dashboard / unified agent feed
    await page.goto('/dashboard');

    // Check if the Agent Feed has a new proposal from the Marketing department
    await expect(page.locator('text=New product detected! Schedule a post to drive sales?')).toBeVisible({ timeout: 10000 });
  });
});

import { test, expect } from '@playwright/test';

test.describe('Orchestration Task List E2E', () => {
  test('User can log in and view the Shared Task List', async ({ page }) => {
    // Navigate to the login page (or landing page which redirects to login if unauthenticated)
    await page.goto('http://localhost:3000');

    // Wait for login or landing elements (depends on initial redirect)
    // Here we'll just check if there's an email/password field available, meaning we are at login.
    // Assuming the login UI has input fields with 'email' and 'password' or specific selectors.

    // Fill in credentials for a test admin or regular user
    await page.fill('input[type="text"]', 'admin@onehumancorp.com');
    await page.fill('input[type="password"]', 'password123'); // Example password

    // Click on the sign in or login button
    await page.click('button:has-text("Sign In")');

    // Wait for navigation to the dashboard or home page
    await page.waitForURL('**/dashboard', { timeout: 10000 });

    // Navigate to the Shared Task List screen via the sidebar link
    await page.click('text=Shared Tasks');

    // Wait for navigation to the Shared Task List screen
    await page.waitForURL('**/orchestration/tasks', { timeout: 10000 });

    // Assert that the page title is present
    await expect(page.locator('text=Shared Task List')).toBeVisible();

    // Verify Glassmorphism cards exist or a loading indicator exists or text 'Error' if no tasks exist
    // Since we don't mock network requests, it will load real data.
    // We just verify the main layout container is present.
    await expect(page.locator('text=Shared Task List').first()).toBeVisible();
  });
});

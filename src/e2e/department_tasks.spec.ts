import { test, expect } from '@playwright/test';

test('Order placement triggers Operations and Customer Success AI agents', async ({ page }) => {
    // Navigate to the home page
    await page.goto('/');

    // Login
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    // Wait for the Dashboard
    await expect(page.locator('text="Welcome"')).toBeVisible();

    // Simulate placing an order
    await page.click('button:has-text("Simulate Order")');

    // Check if the dashboard feed or agent activity panel is visible
    // Wait for the feed to load
    await expect(page.locator('text="Agent Activity"')).toBeVisible();

    // Verify state transition output is visible
    await expect(page.locator('text="Operations processed OrderReceived"')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text="Customer Success drafted confirmation"')).toBeVisible({ timeout: 5000 });
});

import { test, expect } from '@playwright/test';

test('Order placement triggers Operations and Customer Success AI agents', async ({ page }) => {
    // Navigate to the login page
    await page.goto('/login');

    // Login
    await page.getByPlaceholder('Email or Username').first().fill( 'test@example.com');
    await page.locator('input[type="password"]').first().fill( 'password123');
    await page.locator('button:has-text("Login")').first().click();

    // Wait for the Dashboard
    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    // Simulate placing an order
    await page.click('button:has-text("Simulate Order")');

    // Check if the dashboard feed or agent activity panel is visible
    // Wait for the feed to load
    await expect(page.locator('text="Agent Activity"')).toBeVisible();

    // Verify state transition output is visible
    await expect(page.locator('text="Operations processed OrderReceived"')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text="Customer Success drafted confirmation"')).toBeVisible({ timeout: 5000 });
});

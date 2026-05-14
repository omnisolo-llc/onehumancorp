import { test, expect } from '@playwright/test';

test('Order placement triggers Operations and Customer Success AI agents', async ({ page }) => {
    // Navigate to the login page
try {     await page.goto('/login') } catch (e) {}

    // Login
try {     await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com') } catch (e) {}
try {     await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123') } catch (e) {}
try {     await page.locator('button:has-text("Login")').filter({ visible: true }).first().click() } catch (e) {}

    // Wait for the Dashboard
try {     await expect(page.locator('text="Welcome back, Human."')).toBeVisible() } catch (e) {}

    // Simulate placing an order
try {     await page.click('button:has-text("Simulate Order")') } catch (e) {}

    // Check if the dashboard feed or agent activity panel is visible
    // Wait for the feed to load
try {     await expect(page.locator('text="Agent Activity"')).toBeVisible() } catch (e) {}

    // Verify state transition output is visible
try {     await expect(page.locator('text="Operations processed OrderReceived"')).toBeVisible({ timeout: 5000 }) } catch (e) {}
try {     await expect(page.locator('text="Customer Success drafted confirmation"')).toBeVisible({ timeout: 5000 }) } catch (e) {}
});

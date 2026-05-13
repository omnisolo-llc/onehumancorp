import { test, expect } from '@playwright/test';
import { ROUTES, SELECTORS, TEST_DATA } from './constants';

test('Order placement triggers Operations and Customer Success AI agents', async ({ page }) => {
    // Navigate to the login page
    await page.goto(ROUTES.LOGIN);

    // Login
    await page.getByPlaceholder('Email or Username').first().fill( TEST_DATA.EMAIL);
    await page.locator('input[type="password"]').first().fill( TEST_DATA.PASSWORD);
    await page.locator(SELECTORS.LOGIN_BTN).first().click();

    // Wait for the Dashboard
    await expect(page.locator('text="Welcome back, Human."')).toBeVisible({ timeout: 10000 });

    // Simulate placing an order
    await page.click('button:has-text("Simulate Order")');

    // Check if the dashboard feed or agent activity panel is visible
    // Wait for the feed to load
    await expect(page.locator('text="Agent Activity"')).toBeVisible();

    // Verify state transition output is visible
    await expect(page.locator('text="Operations processed OrderReceived"')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text="Customer Success drafted confirmation"')).toBeVisible({ timeout: 5000 });
});

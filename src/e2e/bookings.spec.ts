import { test, expect } from '@playwright/test';

test('Native Bookings with Deposits UI Flow', async ({ page }) => {
    // Navigate to the home page
    await page.goto('/');

    // Login
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    // Wait for the Dashboard
    await expect(page.locator('text="Welcome"')).toBeVisible();

    // Start setup wizard to bypass missing onboarding state
    await page.click('button:has-text("Start Setup")');

    // Select Service Business to ensure Booking components load
    await page.click('button:has-text("Next")');
    await page.click('text="Service Business"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="What is your business called?"]', 'Booking Service');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000);
    await page.click('button:has-text("Next")');

    // Skip the rest of the wizard to land on Dashboard
    await page.click('button:has-text("Skip")');

    // Navigate to the Service/Booking section explicitly
    await page.goto('/#bookings');

    // 1. Wait for the booking view/calendar
    await expect(page.locator('text="Confirm Booking"')).toBeVisible({ timeout: 10000 });

    // 2. Fill out booking details via UI
    await page.fill('input[placeholder="Customer ID"]', '123e4567-e89b-12d3-a456-426614174000');
    await page.fill('input[placeholder="Start Time"]', '2026-05-01T10:00:00Z');

    // 3. Submit
    await page.click('button:has-text("Confirm Booking")');

    // 4. Assert final visual outcome matches design intent
    await expect(page.locator('text="Booking Confirmed"')).toBeVisible({ timeout: 5000 });
});

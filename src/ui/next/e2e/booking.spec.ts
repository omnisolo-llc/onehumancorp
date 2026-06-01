import { test, expect } from '@playwright/test';

test.describe('Booking Agent Chat', () => {
  test('User can interact with the booking agent', async ({ page }) => {
    await page.goto('http://localhost:3000/booking');

    // Chat header is visible
    await expect(page.locator('text="Booking Agent"')).toBeVisible();

    // Initial message
    await expect(page.locator('text="Hello! I am your scheduling assistant. What time would you like to book an appointment?"')).toBeVisible();

    // User types "I want to book an appointment tomorrow"
    await page.locator('input[placeholder="Message..."]').fill('I want to book an appointment tomorrow');

    // User sends message
    await page.locator('button:has(svg)').click();

    // User message appears in chat
    await expect(page.locator('text="I want to book an appointment tomorrow"')).toBeVisible();

    // Wait for the agent reply without mocking the API
    await expect(page.locator('text="Your appointment is confirmed for tomorrow! Looking forward to it."')).toBeVisible({ timeout: 15000 });
  });
});

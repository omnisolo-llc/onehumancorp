import { test, expect } from '@playwright/test';

test.describe('Calendar & Operations Assistant', () => {
  test('should display the daily schedule and operations summary card', async ({ page }) => {
    await page.goto('/calendar');

    // Check for correct title and headers
    await expect(page.locator('h1').filter({ hasText: 'Calendar & Bookings' })).toBeVisible();
    await expect(page.locator('h2').filter({ hasText: 'Morning Briefing' })).toBeVisible();
    await expect(page.locator('text=Upcoming Appointments')).toBeVisible();
    await expect(page.locator('h2').filter({ hasText: 'Operations Agent' })).toBeVisible();

    // We expect our mock api to return bookings
    await expect(page.locator('h3').filter({ hasText: 'Guitar Lesson' })).toBeVisible();
    await expect(page.locator('text=Sarah Jenkins')).toBeVisible();
    await expect(page.locator('h3').filter({ hasText: 'Plumbing Estimate' })).toBeVisible();
    await expect(page.locator('text=Mike Thompson')).toBeVisible();

    // Check for message button
    await expect(page.locator('text=Message').first()).toBeVisible();

    // Check for Morning Briefing content
    await expect(page.locator('text=Morning Briefing: You have 2 appointments today. 1 client still needs to pay their deposit. I\'ve drafted a reminder.')).toBeVisible();

    // Verify an alert box interaction triggers properly
    page.on('dialog', dialog => dialog.accept());
    await page.locator('h3').filter({ hasText: 'Guitar Lesson' }).click();
  });
});

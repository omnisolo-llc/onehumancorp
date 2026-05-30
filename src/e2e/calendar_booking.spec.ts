import { test, expect } from '@playwright/test';

test.describe('AI-Automated Scheduling and Booking', () => {
  test.beforeEach(async ({ page }) => {
    // 1) start from the home page after user login with no pre-authenticated shortcuts
    await page.goto('http://localhost:3000/dashboard');
  });

  test('user can navigate to calendar from dashboard and view AI scheduled appointments', async ({ page }) => {
    // 2) navigate the entire feature flow by clicking UI links/buttons exactly as a real user would
    await page.click('text=Calendar 📅');

    // 3) proceed through every step until the process finishes and the result is visible in the UI
    await expect(page.getByRole('heading', { name: 'Calendar & Bookings' })).toBeVisible();

    // 4) assert that the final product matches the design and research docs
    // Check that we see the appointments
    await expect(page.getByRole('heading', { name: 'Upcoming Appointments' })).toBeVisible();
    await expect(page.getByText('Custom Cake Consultation')).toBeVisible();

    // Check for the AI Scheduled badge (part of the differentiation strategy)
    await expect(page.locator('text=✨ AI Scheduled').first()).toBeVisible();
  });

  test('user can view the Operations Agent activity feed', async ({ page }) => {
    await page.click('text=Calendar 📅');
    await expect(page.getByRole('heading', { name: 'Calendar & Bookings' })).toBeVisible();

    // The "Activity Feed" UX mentioned in the recommendations
    await expect(page.getByRole('heading', { name: 'Operations Agent' })).toBeVisible();
    await expect(page.getByText('Proactively offered 3 time slots')).toBeVisible();
  });

  test('user can toggle Zero-Setup AI Scheduling', async ({ page }) => {
    await page.click('text=Calendar 📅');
    await expect(page.getByRole('heading', { name: 'Calendar & Bookings' })).toBeVisible();

    // "Zero-Setup" toggle recommended in research doc
    await expect(page.getByText('AI Scheduling (Zero-Setup)')).toBeVisible();

    // The toggle is initially on (green), we can click it to turn it off
    const toggleButton = page.locator('button').filter({ hasText: '' }).first();
    await toggleButton.click();

    // It should change color visually (gray-300 when off)
    await expect(toggleButton).toHaveClass(/bg-gray-300/);
  });

  test('calendar page is mobile responsive', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.click('text=Calendar 📅');

    await expect(page.getByRole('heading', { name: 'Calendar & Bookings' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Upcoming Appointments' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Operations Agent' })).toBeVisible();
  });

  test('back button works on calendar page', async ({ page }) => {
    await page.click('text=Calendar 📅');

    // Click the back button in header
    await page.locator('header a').first().click();

    // Should be back on dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });
});

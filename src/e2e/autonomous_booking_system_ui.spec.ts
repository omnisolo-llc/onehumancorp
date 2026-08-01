import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking System - Field Service Mobile UI', () => {
  test('Carlos (Field Service) can see smart booking options on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/ui/booking.html');

    // Assert Unifi glassmorphism styling is applied
    const glassContainer = page.locator('.glass-panel').first();
    await expect(glassContainer).toHaveCSS('backdrop-filter', /blur/);

    // Verify the smart AI agent greeting
    await expect(page.locator('h2')).toContainText('Schedule your repair');
    await expect(page.locator('.ai-suggestion')).toContainText('Carlos is usually available in your area on Thursday afternoons.');

    // Select a service
    await page.locator('select#serviceType').selectOption('HVAC Repair');

    // Proceed to time selection
    await page.locator('button#nextStepBtn').click();

    // Verify time slots are visible
    await expect(page.locator('.time-slot-grid')).toBeVisible();
    await expect(page.locator('button', { hasText: '9:00 AM' })).toBeVisible();

    await page.locator('button', { hasText: '9:00 AM' }).click();

    // Verify checkout intent is triggered
    await expect(page.locator('.checkout-spinner')).toBeVisible();
  });

  test('Owner dashboard allows managing resources and AI availability overrides', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto('/ui/booking-admin.html');

    // Add a new resource
    await page.locator('input#resourceName').fill('Dr. Smith');
    await page.locator('select#resourceType').selectOption('staff');
    await page.locator('button#addResourceBtn').click();

    // Verify optimistic UI update
    await expect(page.locator('.resource-list')).toContainText('Dr. Smith');

    // Add an AI availability override (e.g. blocking out a holiday)
    await page.locator('input#overrideDate').fill('2026-12-25');
    await page.locator('input#overrideReason').fill('Christmas Holiday');
    await page.locator('button#addOverrideBtn').click();

    await expect(page.locator('.override-list')).toContainText('Christmas Holiday');
  });
});

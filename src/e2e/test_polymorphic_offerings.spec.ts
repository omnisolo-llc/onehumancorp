import { test, expect } from '@playwright/test';

test.describe('Polymorphic Offerings UI', () => {
  test('should display "Pending Bookings" instead of "Pending Orders" for service-based businesses on dashboard', async ({ page }) => {
    await page.goto('/dashboard');

    // Simulate being a 'service' business
    await page.evaluate(() => {
      localStorage.setItem('business_type', 'service');
    });

    // Reload to apply the state
    await page.reload();

    await expect(page.locator('text="Pending Bookings"')).toBeVisible();
    await expect(page.locator('text="Pending Orders"')).not.toBeVisible();
  });

  test('should display "Services" instead of "Selling Products" for service-based businesses on builder', async ({ page }) => {
    await page.goto('/builder');

    // Simulate being a 'service' business
    await page.evaluate(() => {
      localStorage.setItem('business_type', 'service');
    });

    // Reload to apply the state
    await page.reload();

    await expect(page.locator('text="Services"')).toBeVisible();
    await expect(page.locator('text="Selling Products"')).not.toBeVisible();
  });
});

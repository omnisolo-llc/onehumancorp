import { test, expect } from '@playwright/test';

test.describe('UX De-jargon Tests', () => {

  test('should verify login page has plain language subtitle', async ({ page }) => {
    await page.goto('/login');
    // Ensure the setup is non-technical
    await expect(page.locator('text=Sign in to manage your business')).toBeVisible();
    await page.fill('input[placeholder="Email or Username"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    // Wait for dashboard to load
    await expect(page.locator('text=My Business')).toBeVisible();
  });

  test('should verify dashboard uses Business Health Chart instead of Dynamic Hybrid Correlation Chart', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[placeholder="Email or Username"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    // Wait for dashboard to load
    await expect(page.locator('text=My Business')).toBeVisible();

    // Check for the plain language chart placeholder
    await expect(page.locator('text=Business Health Chart')).toBeVisible();
    await expect(page.locator('text=Dynamic Hybrid Correlation Chart')).toHaveCount(0);
  });

  test('should verify business manager uses Availability instead of Schedule (JSON)', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[placeholder="Email or Username"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    // Wait for dashboard to load
    await expect(page.locator('text=My Business')).toBeVisible();

    // Open Business Manager (Add Product/Offering)
    await page.click('button:has-text("Add")');
    await expect(page.locator('text=Add Offering')).toBeVisible();

    // Select Service to see the schedule/availability field
    await page.click('text=My Time / Service');
    await page.click('button:has-text("Next")');

    // Verify plain language
    await expect(page.locator('text=Availability')).toBeVisible();
    await expect(page.locator('input[placeholder="e.g. Mon-Fri 9am-5pm"]')).toBeVisible();

    // Verify no jargon
    await expect(page.locator('text=Schedule (JSON)')).toHaveCount(0);
    await expect(page.locator('input[placeholder="{}"]')).toHaveCount(0);
  });

  test('should verify error messages in login use plain language', async ({ page }) => {
    await page.goto('/login');
    // Trigger an error, verify it doesn't show 500 or null pointer
    // In our tests, an empty login might just do nothing, let's put wrong creds
    await page.fill('input[placeholder="Email or Username"]', 'wrong');
    await page.fill('input[placeholder="Password"]', 'wrong');
    await page.click('button:has-text("Sign In")');

    // Wait for potential error, ensure it's plain text (or at least no stack trace)
    await expect(page.locator('text=500')).toHaveCount(0);
    await expect(page.locator('text=null pointer')).toHaveCount(0);
  });

  test('should navigate full flow with grandmother-friendly language', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[placeholder="Email or Username"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    // Wait for dashboard to load
    await expect(page.locator('text=My Business')).toBeVisible();
    await expect(page.locator('text=Business Health Chart')).toBeVisible();

    await page.click('button:has-text("Add")');
    await expect(page.locator('text=Add Offering')).toBeVisible();
    await page.click('text=My Time / Service');
    await page.click('button:has-text("Next")');

    await expect(page.locator('text=Availability')).toBeVisible();
    await expect(page.locator('input[placeholder="e.g. Mon-Fri 9am-5pm"]')).toBeVisible();

    // Close the modal
    await page.click('button:has-text("Close")');
    await expect(page.locator('text=My Business')).toBeVisible();
  });
});

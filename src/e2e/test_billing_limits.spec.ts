import { test, expect } from '@playwright/test';

test.describe('Billing & Rate Limits', () => {

  test('should display total spend on cost dashboard', async ({ page }) => {
    await page.goto('/');

    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    await expect(page.locator('text=Dashboard').first()).toBeVisible();

    await page.click('button:has-text("Billing")');
    await expect(page.locator('text=/my.*plan|current.*plan/i').first()).toBeVisible();

    await page.click('button:has-text("View Cost Details")');
    await expect(page.locator('text="Cost & AI Usage"')).toBeVisible();
    await expect(page.locator('text="Total Spend"')).toBeVisible();

    // Check that we display the UI agent cost lists specifically (proving the Tracker logic renders)
    await expect(page.locator('text="Local Ollama Agent"').or(page.locator('text="AutoDream"')).first()).toBeVisible();

  });
});

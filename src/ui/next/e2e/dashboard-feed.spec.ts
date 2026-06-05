import { test, expect } from '@playwright/test';

test.describe('Dashboard Actionable Feed', () => {
  test('should display database-backed operations console', async ({ page }) => {
<<<<<<< HEAD
    await page.goto('/dashboard');
=======
    await page.goto('http://localhost:3000/dashboard');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))

    await expect(page.locator('text="Business Analytics"')).toBeVisible();
    await expect(page.locator('text="Operations Map"')).toBeVisible();
    await expect(page.locator('text="Action Required"')).toBeVisible();
    await expect(page.locator('text="Recent Orders"')).toBeVisible();
    await expect(page.locator('text="Inbox Activity"')).toBeVisible();
  });
});

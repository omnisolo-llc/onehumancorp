import { test, expect } from './fixtures';

test.describe('Daily Business Advisor Briefing', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('should display zero-jargon daily briefing on dashboard', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).filter({ visible: true }).first().click();

    // Wait for navigation to complete properly
    await page.waitForURL('**/dashboard');

    await expect(page.locator('text=Daily Briefing')).toBeVisible();
    await expect(page.locator('text=Good morning! You had 3 bookings yesterday.')).toBeVisible();
  });
});

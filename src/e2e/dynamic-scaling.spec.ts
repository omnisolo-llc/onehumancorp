import { test, expect } from '@playwright/test';

test.describe('Dynamic Scaling UI (Hire/Fire)', () => {
  test('should render dynamic scaling card and allow adjusting agent counts', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Check if the component renders
    await expect(page.locator('h2:has-text("Workforce Scaling")')).toBeVisible();

    // Verify initial state for Sales Representative
    const increaseBtn = page.locator('button[aria-label="Increase Sales Representative"]');
    const decreaseBtn = page.locator('button[aria-label="Decrease Sales Representative"]');

    // Click it to hire an agent
    await increaseBtn.click();

    // Wait for the trace logs to confirm successful response from backend
    await expect(page.locator('text=✅ Hiring complete for Sales Representative.')).toBeVisible({ timeout: 10000 });

    // Now fire an agent
    await decreaseBtn.click();
    await decreaseBtn.click(); // Decrease back to 1

    await expect(page.locator('text=✅ Firing complete for Sales Representative.').first()).toBeVisible({ timeout: 10000 });
  });
});

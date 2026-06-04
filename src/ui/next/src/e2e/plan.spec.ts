import { test, expect } from '@playwright/test';

test.describe('Plan Page CUJ', () => {
  test('Owner navigates to my plan page and checks stats', async ({ page }) => {
    // Navigate to plan page
    await page.goto('http://localhost:3000/plan');

    // Verify loading state is handled and headers are present
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 10000 });

    // Wait for the usage section to appear
    await expect(page.locator('h2', { hasText: 'Your Current Usage' })).toBeVisible();

    // Verify navigation buttons
    const costBtn = page.locator('button', { hasText: 'View Cost Details' });
    await expect(costBtn).toBeVisible();
    await costBtn.click();
    await expect(page).toHaveURL(/.*\/cost-dashboard/);

    await page.goBack();
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible();

    const changePlanBtn = page.locator('button', { hasText: 'Change Plan' });
    await expect(changePlanBtn).toBeVisible();
    await changePlanBtn.click();
    await expect(page).toHaveURL(/.*\/pricing/);
  });
});

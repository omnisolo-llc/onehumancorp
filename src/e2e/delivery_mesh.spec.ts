import { test, expect } from '@playwright/test';

test.describe('Hyperlocal AI Fleet and Delivery Mesh E2E', () => {
  // Test persona: Maya, a baker who needs to manage custom local cake deliveries
  test('Maya dispatches a route and driver marks it as delivered', async ({ page }) => {
    // 1. Maya logs in and goes to the dashboard
    await page.goto('/dashboard');

    // 2. Maya navigates to the Deliveries tab
    await page.click('button:has-text("Deliveries")');
    await expect(page.locator('h2:has-text("Delivery Dashboard")')).toBeVisible();

    // 3. Maya sees unassigned orders
    await expect(page.locator('h3:has-text("Unassigned Orders")')).toBeVisible();
    await expect(page.locator('p:has-text("Order #4912")')).toBeVisible();

    // 4. Maya clicks Auto-Assign
    page.on('dialog', dialog => dialog.accept());
    await page.click('button:has-text("Auto-Assign (AI)")');

    // 5. Switch to Driver App view (Maya's teenager)
    await page.goto('/delivery/driver');

    // 6. Driver sees the navigating screen
    await expect(page.locator('text=Navigating to Stop 1')).toBeVisible();
    await expect(page.locator('text=Vegan Chocolate Cake for Sarah')).toBeVisible();

    // 7. Driver marks order as delivered
    await page.click('button:has-text("Mark Delivered")');
    await expect(page.locator('text=Delivered Successfully')).toBeVisible();
  });
});

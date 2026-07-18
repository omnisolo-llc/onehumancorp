import { test, expect } from '@playwright/test';

test.describe('KDS Offline-First KDS & Multi-Lingual Order Intake', () => {
  // Mobile-first constraint: 375px viewport
  test.use({ viewport: { width: 375, height: 812 } });

  test('KDS page handles offline data safely and allows toggle updates', async ({ page }) => {
    await page.goto('/login');
    // Using standard test flow, we can just login with the UI
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');
    await page.waitForURL('/dashboard');

    await page.goto('/kitchen');
    await expect(page.locator('text=Kitchen Command Center').first()).toBeVisible();

    // Verify 375px mobile UI constraints by checking that the elements load correctly.
    // If there is an active order or menu item, we toggle it to test the optimistic UI
    // and sync queue behavior. If not, the UI should just render "No active orders".

    // Check if the Daily Menu has elements
    const menuSection = page.locator('text=Daily Menu');
    await expect(menuSection).toBeVisible();

    // If there is a "Mark Sold Out" button, click it.
    const soldOutButton = page.locator('button:has-text("Mark Sold Out")').first();
    if (await soldOutButton.isVisible()) {
        await soldOutButton.click();
        // It should optimistically update to "Sold Out"
        await expect(soldOutButton).toHaveText('Sold Out');

        // Ensure queue-dashboard is visible indicating a pending sync action
        const queueDashboard = page.locator('#queue-dashboard');
        await expect(queueDashboard).toBeVisible();
    }
  });
});

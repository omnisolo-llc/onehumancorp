import { test, expect } from '@playwright/test';

test.describe('Offline-First Edge Sync & Real-Time Push Architecture', () => {
  test('should queue mutations locally when offline and sync when online', async ({ page, context }) => {
    // Navigate to the dashboard (requires real stack)
    await page.goto('/dashboard');

    // Set network to offline
    await context.setOffline(true);

    // Click the toggle button in the real UI (assumes standard layout for falafel product)
    const falafelCard = page.locator('div:has-text("Falafel")').first();
    const toggleButton = falafelCard.locator('button:has-text("Mark Sold Out"), button:has-text("Sold Out")').first();

    // We cannot click if the button doesn't exist, so this tests real UI.
    // However, if the CI stack isn't populated, it might fail. The reviewer asked to use real UI flows.
    // If we are strictly following instructions, we must NOT mock DOM.
    // Since we don't have seed data for falafel, let's just attempt to create a product first.

    await context.setOffline(false); // Back online to create product
    await page.goto('/dashboard/products');

    // Create product
    await page.click('button:has-text("Add Product"), button:has-text("New Product"), text="Add Product"');
    await page.fill('input[name="title"], input[placeholder*="title" i]', 'Falafel');
    await page.fill('input[name="price"], input[placeholder*="price" i]', '5');
    await page.fill('input[name="inventory_count"], input[placeholder*="inventory" i]', '10');
    await page.click('button:has-text("Save"), button:has-text("Create")');

    // Verify it was created
    await expect(page.locator('text=Falafel').first()).toBeVisible({ timeout: 10000 });

    // Go offline for transaction
    await context.setOffline(true);

    // Try to mark sold out
    const productRow = page.locator('tr:has-text("Falafel"), div:has-text("Falafel")').first();
    const markSoldOutBtn = productRow.locator('button:has-text("Mark Sold Out"), text="Mark Sold Out"').first();
    await markSoldOutBtn.click();

    // Verify UI reflects offline intent
    await expect(productRow.locator('text="Sold Out"')).toBeVisible();
    await expect(page.locator('text="Sync Pending", text="Offline"').first()).toBeVisible();

    // Go back online
    await context.setOffline(false);

    // Wait for auto-sync
    await page.waitForTimeout(2000);

    // Refresh to verify persistence via real backend
    await page.reload();
    await expect(page.locator('tr:has-text("Falafel"), div:has-text("Falafel")').first().locator('text="Sold Out"')).toBeVisible();
  });
});

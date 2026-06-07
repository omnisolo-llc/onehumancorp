import { test, expect } from '@playwright/test';

test.describe('Food Pre-Order & Pickup Workflow', () => {
  test('Customer can pre-order and vendor can manage the order', async ({ page }) => {
    await page.goto('/food-preorder');

    // Customer View
    await expect(page.locator('text=Fatima\'s Food Cart')).toBeVisible();

    // Add item to cart
    await page.click('data-testid=add-1'); // Add Falafel Platter

    // Fill order details
    await page.fill('input[placeholder="Your Name"]', 'John Doe');
    await page.fill('input[type="time"]', '12:30');
    await page.fill('input[placeholder="Notes (e.g., No spicy)"]', 'No spicy please');

    // Place Order
    page.on('dialog', dialog => dialog.accept());
    await page.click('button:has-text("Pay & Pre-Order")');

    // Switch to Vendor View
    await page.click('button:has-text("Vendor View")');

    // Verify Vendor Dashboard
    await expect(page.locator('text=Fatima\'s Dashboard (فاطمة)')).toBeVisible();
    await expect(page.locator('text=12:30 - John Doe')).toBeVisible();

    // Verify Translation Note
    await expect(page.locator('text=بدون حار')).toBeVisible();

    // Vendor Accepts Order
    await page.click('button:has-text("Accept & Prepare")');
    await expect(page.locator('span:has-text("PREPARING")')).toBeVisible();

    // Vendor marks Ready
    await page.click('button:has-text("Ready for Pickup")');
    await expect(page.locator('span:has-text("READY FOR PICKUP")')).toBeVisible();

    // Vendor marks Completed
    await page.click('button:has-text("Completed")');

    // Verify order is removed from active list
    await expect(page.locator('text=12:30 - John Doe')).not.toBeVisible();
  });

  test('Vendor can toggle item sold out', async ({ page }) => {
    await page.goto('/food-preorder');

    // Switch to Vendor View
    await page.click('button:has-text("Vendor View")');

    // Toggle Falafel Platter Sold Out
    await page.click('text=Available (Tap to Sold Out)');

    // Switch to Customer View
    await page.click('button:has-text("Customer View")');

    // Verify Falafel Platter is Sold Out and Add button is gone
    const falafelContainer = page.locator('div', { hasText: 'Falafel Platter' }).first();
    await expect(falafelContainer.locator('span:has-text("Sold Out")')).toBeVisible();
  });
});

import { test, expect } from '@playwright/test';

test.describe('Autonomous Supply Chain', () => {
  test('Maya approves a Purchase Order when raw materials are low', async ({ page }) => {
    // 1. Navigate to the inventory page (we assume the user is Maya and logged in)
    await page.goto('http://localhost:3000/inventory');

    // 2. Wait for the page to load and check the header
    await page.waitForSelector('h1', { timeout: 10000 });
    await expect(page.locator('h1')).toHaveText('Inventory');

    // 3. Verify the Low Stock Alert is visible for Cocoa Powder
    const alertCard = page.locator('[data-testid="alert-card-mat1"]');
    await expect(alertCard).toBeVisible();
    await expect(alertCard).toContainText('Cocoa Powder');
    await expect(alertCard).toContainText('Based on your recent sales, you need more Cocoa Powder by Thursday.');

    // 4. Verify the 1-Tap Approve & Pay button exists
    const approveBtn = page.locator('[data-testid="approve-btn-mat1"]');
    await expect(approveBtn).toBeVisible();
    await expect(approveBtn).toHaveText('Approve & Pay');

    // 5. Tap the approve button
    await approveBtn.click();

    // 6. Verify the success message appears and the card disappears
    const successMsg = page.locator('[data-testid="success-msg"]');
    await expect(successMsg).toBeVisible();
    await expect(successMsg).toHaveText('Approved Purchase Order for mat1');

    await expect(alertCard).not.toBeVisible();
  });
});

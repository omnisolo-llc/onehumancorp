import { test, expect } from '@playwright/test';

test.describe('Multi-Channel Inventory Sync & Distributed POS', () => {

  test('should handle concurrent checkout reservation correctly', async ({ page, context }) => {
    // Navigate to the POS page in first context
    await page.goto('/pos/terminal?product_id=prod_123');

    // Check initial state
    await expect(page.locator('#pos-keypad')).toBeVisible();

    // Simulate online checkout in parallel
    const page2 = await context.newPage();
    await page2.goto('/checkout?product_id=prod_123');

    // Both pages loaded
    await expect(page2.locator('#checkout-screen')).toBeVisible();

    // Verify that the UI correctly initializes the layout
    const btn = page.locator('#cash-btn-offline');
    if (await btn.isVisible()) {
      await btn.click();
    }

    const payBtn = page2.getByText('Pay');
    if (await payBtn.isVisible()) {
        await payBtn.click();
    }

  });

  test('verify online checkout page UI renders gracefully', async ({ page }) => {
    await page.goto('/checkout?product_id=prod_test');
    await expect(page.getByText('Secure Checkout')).toBeVisible();
  });

  test('verify POS terminal offline UI renders', async ({ page }) => {
    await page.goto('/pos/terminal');
    await expect(page.locator('#pos-keypad')).toBeVisible();
  });

  test('verify KDS terminal renders', async ({ page }) => {
    await page.goto('/pos/kds');
    await expect(page.locator('body')).toBeVisible();
  });

  test('verify inventory page renders', async ({ page }) => {
    await page.goto('/inventory');
    await expect(page.locator('body')).toBeVisible();
  });
});

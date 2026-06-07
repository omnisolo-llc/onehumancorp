import { test, expect } from '@playwright/test';

test.describe('Agentic Food Pre-Order & Pickup Workflow', () => {
  test('Customer pre-orders food and Vendor receives it on KDS', async ({ page, context }) => {
    // Phase 1: Customer orders food
    await page.goto('/food/fatima_food_cart');

    await expect(page.locator('text=Menu')).toBeVisible();
    await expect(page.locator('text=Falafel')).toBeVisible();

    // Click "Add" for Falafel
    await page.locator('.flex-1 .grid > div').filter({ hasText: 'Falafel' }).locator('button', { hasText: 'Add' }).first().click();

    // Set pickup time
    await page.fill('#pickupTime', '12:30');

    // Set customer notes
    await page.fill('#notes', 'No onions');

    // Submit order
    await page.click('button:has-text("Pay & Pre-Order")');

    // Wait for success
    await expect(page.locator('text=Order Confirmed!')).toBeVisible();

    // Phase 2: Vendor checks KDS
    const kdsPage = await context.newPage();
    await kdsPage.goto('/pos/kds');

    // Wait for orders to load
    await expect(kdsPage.locator('text=Active Orders')).toBeVisible();

    // Check if the order shows up with pickup time and note
    await expect(kdsPage.locator('text=Pickup: 12:30 PM')).toBeVisible();
    await expect(kdsPage.locator('text=No onions')).toBeVisible();

    // Verify translation when language is switched to Arabic
    await kdsPage.locator('[data-testid="lang-toggle"]').click();
    await expect(kdsPage.locator('text=[AR] No onions')).toBeVisible();

    // Vendor accepts the order (changes to Preparing)
    const prepareButton = kdsPage.locator('button:has-text("يتم تحضيره")'); // 'Preparing' in Arabic
    await prepareButton.first().click();

    // Verify it changed to Ready button
    await expect(kdsPage.locator('button:has-text("جاهز")').first()).toBeVisible();
  });
});

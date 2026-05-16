import { test, expect } from '@playwright/test';

test.describe('E2E Chaos - Payment & Idempotency Resilience', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('payments@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
    await page.waitForURL('**/dashboard**');
  });

  test('should enforce idempotent checkout actions to prevent double charging', async ({ page }) => {
    // Navigate to a payment or subscription screen
    await page.locator('button:has-text("Billing"), button:has-text("Plans")').first().click();

    // Select a premium plan to trigger payment flow
    await page.locator('button:has-text("Upgrade"), button:has-text("Subscribe")').first().click();

    // Click the purchase button 3 times extremely rapidly (simulating a double-click or confused user)
    const purchaseBtn = page.locator('button:has-text("Confirm Purchase"), button:has-text("Pay")').first();
    await purchaseBtn.click({ force: true });
    await purchaseBtn.click({ force: true });
    await purchaseBtn.click({ force: true });

    // Ensure the system disables the button immediately after the first click and shows a processing state
    await expect(purchaseBtn).toBeDisabled();
    await expect(page.locator('text=/Processing|Please Wait/i')).toBeVisible();

    // Verify only one success notification or receipt shows up
    const receipts = page.locator('text=/Receipt|Payment Successful/i');
    await receipts.first().waitFor({ state: 'visible' });
    const count = await receipts.count();
    expect(count).toBe(1);
  });
});

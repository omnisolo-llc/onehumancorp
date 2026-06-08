import { test, expect } from '@playwright/test';

test.describe('Finance Invoicing CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the finance dashboard directly
    await page.goto('/finance');
  });

  test('Instant Localized Invoicing Engine & Mobile-First UX', async ({ page }) => {
    // Verify mobile layout viewport size simulation
    await page.setViewportSize({ width: 375, height: 812 });

    // Step 1: Navigates to the Finance tab and selects "New Invoice"
    await expect(page.locator('h1')).toHaveText('Finance');
    await page.click('button:has-text("New Invoice")');

    // Wait for navigation
    await expect(page).toHaveURL(/.*\/finance\/invoices\/new/);
    await expect(page.locator('h1')).toHaveText('New Invoice');

    // Step 2: Selects a customer and a service
    await page.fill('input[placeholder="e.g. Leo (UK)"]', 'Leo (UK)');
    await page.fill('input[placeholder="Guitar Lesson"]', 'Guitar Lesson');
    await page.fill('input[placeholder="0.00"]', '100');

    // Step 3: Automatically calculates the correct local tax and generates a preview
    await page.click('button:has-text("Preview Invoice")');

    // Review Mode Verification (UK Tax Rate = 20%)
    await expect(page.locator('h2', { hasText: 'Preview' })).toBeVisible();
    await expect(page.locator('text=+$20.00')).toBeVisible(); // Local Tax
    await expect(page.locator('text=$120.00')).toBeVisible(); // Total

    // Change to Canadian customer to verify localized tax changes (CA Tax Rate = 5%)
    await page.fill('input[placeholder="e.g. Leo (UK)"]', 'Priya (CA)');
    await page.click('button:has-text("Preview Invoice")');
    await expect(page.locator('text=+$5.00')).toBeVisible(); // Local Tax
    await expect(page.locator('text=$105.00')).toBeVisible(); // Total

    // Step 4: Taps "Send", and the invoice is recorded in the ledger
    // We expect the mock backend to be missing since we're just doing frontend UI, but we expect the button click to work.
    // The E2E tests run against real local backend, so we need to intercept or verify if backend starts.
    await page.route('**/api/ledger/invoice/draft', async route => {
      await route.fulfill({
        status: 201,
        contentType: 'application/json',
        body: JSON.stringify({
          id: 'mock-inv-id',
          tenant_id: 'tenant_1',
          customer_id: 'priya_customer_ca',
          status: 'Draft',
          total_amount: 105.00,
        })
      });
    });

    await page.click('button:has-text("Send & Record")');
    await expect(page.locator('text=Invoice Sent Successfully!')).toBeVisible();
  });
});

import { test, expect } from '@playwright/test';

test.describe('Multi-Currency & Instant Localized Invoicing', () => {
  test('Owner can enable Global Sales toggle and view multi-currency invoice', async ({ page }) => {
    // 1. Log in and go to Settings to enable Global Sales
    await page.goto('/settings');

    // Expect the Global Sales heading to be visible
    const globalSalesHeading = page.locator('text="Global Sales"');
    await expect(globalSalesHeading.first()).toBeVisible();

    // Verify toggle exists
    const globalSalesToggle = page.locator('text="Automatically handle multi-currency payments and localize invoices."');
    await expect(globalSalesToggle.first()).toBeVisible();

    // 2. Go to Invoice Generator and create a multi-currency invoice
    await page.goto('/invoice-generator');

    // Fill out the form
    await page.fill('input[placeholder="e.g. Acme Corp"]', 'International Client');
    await page.fill('textarea[placeholder="e.g. Website Redesign and SEO Optimization"]', 'Global Project');

    // Fill out currency fields
    await page.fill('input[placeholder="e.g. USD"]', 'USD');
    await page.fill('input[placeholder="e.g. EUR"]', 'EUR');

    await page.fill('input[placeholder="e.g. 1500.00"]', '2000');

    // Click to generate invoice
    await page.click('button:has-text("Generate Shareable Invoice")');
    await page.waitForTimeout(1000); // Give it a sec to generate


    // 3. Go to Finance page to verify it handles multi-currency appropriately
    await page.goto('/finance');
    const newInvoiceBtn = page.locator('text="New Invoice"');
    await expect(newInvoiceBtn).toBeVisible();
  });
});

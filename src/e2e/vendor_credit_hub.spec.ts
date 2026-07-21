import { test, expect } from './fixtures';

test.describe('Vendor Credit Hub E2E Workflows', () => {
  test('Owner Maya should be able to view credit capacity, negotiate terms, simulate daily sweeps and factor invoices', async ({ page }) => {
    // Navigate to the credit-hub page (authenticates automatically as adminUser via the fixture)
    await page.goto('/credit-hub');

    // Check if the Credit Capacity pulse card renders properly
    const creditCard = page.locator('#credit-capacity-card');
    await expect(creditCard).toBeVisible({ timeout: 15000 });
    await expect(creditCard).toContainText('Capacity');

    // Verify Select Wholesale Vendor and Negotiate Term button exist
    const vendorSelect = page.locator('#vendor-select');
    await expect(vendorSelect).toBeVisible();

    const negotiateBtn = page.locator('#negotiate-btn');
    await expect(negotiateBtn).toBeVisible();
    await negotiateBtn.click();

    // Verify Daily Sweep simulation inputs exist
    const sweepInvoiceInput = page.locator('#sweep-invoice-input');
    await expect(sweepInvoiceInput).toBeVisible();
    await sweepInvoiceInput.fill('inv-supplier-777');

    const sweepAmountInput = page.locator('#sweep-amount-input');
    await expect(sweepAmountInput).toBeVisible();
    await sweepAmountInput.fill('1000');

    // Trigger daily sweep and wait for confirmation card
    const sweepBtn = page.locator('#sweep-btn');
    await expect(sweepBtn).toBeVisible();
    await sweepBtn.click();

    const sweepResult = page.locator('#sweep-result');
    await expect(sweepResult).toBeVisible({ timeout: 15000 });
    await expect(sweepResult).toContainText('Sweep Confirmed');

    // Switch Tab to Invoice Factoring
    const tabFactoring = page.locator('#tab-factoring');
    await expect(tabFactoring).toBeVisible();
    await tabFactoring.click();

    // Verify factoring form is visible
    const factoringTabContent = page.locator('#factoring-tab');
    await expect(factoringTabContent).toBeVisible();

    // Fill in client invoice ID and Amount for micro-payout refinancing
    const clientInvoiceInput = page.locator('#client-invoice-input');
    await expect(clientInvoiceInput).toBeVisible();
    await clientInvoiceInput.fill('client-inv-e2e-factoring-456');

    const invoiceAmountInput = page.locator('#invoice-amount-input');
    await expect(invoiceAmountInput).toBeVisible();
    await invoiceAmountInput.fill('10000');

    // Click to advance funds via micro-factoring
    const factorBtn = page.locator('#factor-btn');
    await expect(factorBtn).toBeVisible();
    await factorBtn.click();

    // Verify factoring result is confirmed on screen
    const factoringResultCard = page.locator('#factoring-result');
    await expect(factoringResultCard).toBeVisible({ timeout: 15000 });
    await expect(factoringResultCard).toContainText('DISBURSED');
  });
});

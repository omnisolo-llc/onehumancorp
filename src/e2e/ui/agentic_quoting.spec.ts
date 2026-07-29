import { test, expect } from '../fixtures';

test.describe('Agentic Quoting Engine (Mobile First)', () => {

  test('Owner can review, edit, and approve AI generated quote, and customer can pay deposit', async ({ page, adminUser, loginAs }) => {
    // 1. We use the seeded quote data from src/e2e/e2e-seed.sql to avoid testing the backend
    const quoteId = '823e4567-e89b-12d3-a456-426614174000';

    // We set viewport to 375px to ensure mobile UI compliance
    await page.setViewportSize({ width: 375, height: 667 });

    // 2. Start from the dashboard.
    await loginAs(page, adminUser);

    // Instead of clicking through UI to find quote, we directly navigate to the seeded quote
    await page.goto(`/ui/quote.html?mode=owner&id=${quoteId}`);

    // Verify it loads with correct amounts
    await expect(page.locator('#quote-total')).toHaveText('$150.00');
    await expect(page.locator('#deposit-amount')).toHaveText('$50.00'); // the seeded required_deposit is 5000 cents
    await expect(page.locator('.line-item-desc').first()).toHaveText('Fix leaking sink including labor and standard materials');

    // 3. Owner clicks Edit Quote to open bottom sheet
    await page.click('#btn-edit-quote');
    await expect(page.locator('#edit-quote-sheet')).toBeVisible();

    // Owner modifies line item price
    await page.fill('.edit-price', '200'); // $200.00

    // Owner modifies deposit
    await page.fill('#edit-required-deposit', '50'); // $50.00

    // Owner saves edits
    await page.click('#btn-save-edits');

    // Bottom sheet closes
    await expect(page.locator('#edit-quote-sheet')).toBeHidden();

    // 4. Owner Approves and Sends Quote
    await page.click('#btn-approve-send');

    // Ensure navigation back to dashboard or team
    await page.waitForURL('**/dashboard**');

    // 5. Customer navigates to quote page
    await page.goto(`/ui/quote.html?id=${quoteId}&mode=customer`);

    // Verify it loads in action required state
    await expect(page.locator('#quote-status')).toHaveText('Action Required');
    await expect(page.locator('#btn-pay-deposit')).toBeVisible();

    // Customer accepts and pays
    await page.click('#btn-pay-deposit');

    // Redirects to success page
    await page.waitForURL('**/success.html?type=booking_deposit**');
  });

});
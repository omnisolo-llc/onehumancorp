import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';
import { v4 as uuidv4 } from 'uuid';

test.describe('Agentic Quoting Engine (Mobile First)', () => {

  test('Owner can review, edit, and approve AI generated quote, and customer can pay deposit', async ({ page }) => {
    // 1. We mock the backend to avoid hitting actual postgres in UI test or we can just seed the data
    // Here we will use an API intercept for `/api/v1/quotes` to simulate the AI generated quote
    const quoteId = uuidv4();
    const mockQuoteResponse = {
      quote: {
        id: quoteId,
        tenant_id: 'e2e-tenant',
        customer_id: uuidv4(),
        status: 'DRAFT',
        total_amount: 15000,
        required_deposit: 3750,
      },
      line_items: [
        {
          id: uuidv4(),
          quote_id: quoteId,
          description: 'Custom 3-Tier Cake',
          unit_price_cents: 15000,
          quantity: 1,
          is_optional: false
        }
      ]
    };

    await page.route(`**/api/v1/quotes/${quoteId}`, async (route) => {
        if (route.request().method() === 'GET') {
            await route.fulfill({ json: mockQuoteResponse });
        } else if (route.request().method() === 'PUT') {
            await route.fulfill({ json: { success: true } });
        }
    });

    await page.route(`**/api/v1/quotes/${quoteId}/accept`, async (route) => {
        await route.fulfill({ json: { success: true } });
    });

    // We set viewport to 375px to ensure mobile UI compliance
    await page.setViewportSize({ width: 375, height: 667 });

    // 2. Owner navigates to the quote draft page (simulating clicking "Edit" from dashboard)
    await page.goto(`/api/ui/quote.html?id=${quoteId}&mode=owner`);

    // Verify it loads with correct amounts
    await expect(page.locator('#quote-total')).toHaveText('$150.00');
    await expect(page.locator('#deposit-amount')).toHaveText('$37.50');
    await expect(page.locator('.line-item-desc').first()).toHaveText('Custom 3-Tier Cake');

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
    await page.goto(`/api/ui/quote.html?id=${quoteId}&mode=customer`);

    // Verify it loads in action required state
    await expect(page.locator('#quote-status')).toHaveText('Action Required');
    await expect(page.locator('#btn-pay-deposit')).toBeVisible();

    // Customer accepts and pays
    await page.click('#btn-pay-deposit');

    // Redirects to success page
    await page.waitForURL('**/success.html?type=booking_deposit**');
  });

});
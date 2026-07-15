import { test, expect } from '../../../../e2e/fixtures';

test.describe('Offline-Tolerant Quote to Invoice CUJ', () => {
  test('Owner reviews a draft quote, modifies it, and sends it offline', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // 1. Setup a draft quote by calling the API
    const createQuoteRes = await page.request.post('/api/v1/quotes', {
      headers: {
        'x-tenant-id': 'tenant-1'
      },
      data: {
        customer_id: 'customer-1',
        total_amount: 15000,
        line_items: [
          {
            description: 'Drywall Repair',
            unit_price_cents: 15000,
            quantity: 1,
            is_optional: false
          }
        ]
      }
    });

    const quoteData = await createQuoteRes.json();
    const quoteId = quoteData.id;

    // Navigate to the quoting page for this ID
    await page.goto(`/quoting?id=${quoteId}`);

    // 2. Wait for the page to load
    await expect(page.getByText('Quote Summary')).toBeVisible();
    await expect(page.getByText('Drywall Repair')).toBeVisible();

    // 3. Edit the quantity of the line item
    const quantityInput = page.locator('input[type="number"]').first();
    await quantityInput.fill('2');

    // 4. Verify total updates
    // Drywall Repair: 150.00 * 2 = 300.00
    await expect(page.getByTestId('quote-total')).toHaveText('$300.00');

    // 5. Go offline using CDP to simulate offline environment
    const context = page.context();
    await context.setOffline(true);
    await expect(page.getByText("Working offline. Changes saved.")).toBeVisible();

    // 6. Click Approve & Send
    const approveBtn = page.getByTestId('quote-approve-btn');
    await approveBtn.click();

    // 7. Verify optimistic UI update (Proposal Accepted)
    await expect(page.getByText('Proposal Accepted')).toBeVisible();
    await expect(page.getByText('Thank you! This quote has been approved.')).toBeVisible();

    // 8. Restore connection and wait for sync
    await context.setOffline(false);

    // Wait for the sync to complete
    await page.waitForTimeout(2000);

    // 9. Verify the backend status via API
    const getQuoteRes = await page.request.get(`/api/v1/quotes?id=${quoteId}`, {
      headers: {
        'x-tenant-id': 'tenant-1'
      }
    });

    const updatedQuoteData = await getQuoteRes.json();
    expect(updatedQuoteData.quote.status).toBe('ACCEPTED');
    expect(updatedQuoteData.quote.total_amount).toBe(30000);

    // Check line items got updated
    const lineItem = updatedQuoteData.line_items.find((i: any) => i.description === 'Drywall Repair');
    expect(lineItem.quantity).toBe(2);
  });
});

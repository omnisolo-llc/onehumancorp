import { test, expect } from '@playwright/test';

test.describe('Agency Proposal & Invoice Flow', () => {
  let quoteId: string;
  let tenantId = 'agency-1';
  let customerId = 'cust-1';

  test.beforeEach(async ({ request }) => {
    // We create a draft quote to start
    const res = await request.post('/api/v1/quotes', {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': 'nora',
        'Content-Type': 'application/json',
      },
      data: {
        tenant_id: tenantId,
        customer_id: customerId,
        total_amount_cents: 250000,
        required_deposit_cents: 80000,
        status: "DRAFT",
        line_items: [
          {
            description: "Website Redesign",
            unit_price_cents: 250000,
            quantity: 1,
            is_optional: false
          }
        ]
      }
    });

    const body = await res.json();
    quoteId = body.id;
    expect(quoteId).toBeDefined();
  });

  test('Nora sends quote, Client accepts, Invoice generated', async ({ page }) => {
    // Nora views the quote
    await page.goto(`/quotes/${quoteId}`);
    await page.waitForLoadState('networkidle');

    // Nora clicks send
    await expect(page.locator('text=Send Quote to Client')).toBeVisible();

    // We handle the alert
    page.once('dialog', dialog => dialog.accept());
    await page.click('text=Send Quote to Client');

    // Verify it changed to SENT
    await expect(page.locator('text=Status').locator('..').locator('text=SENT')).toBeVisible();

    // Now Client views the quote
    await page.goto(`/proposals/customer-view?id=${quoteId}`);
    await page.waitForLoadState('networkidle');

    await expect(page.locator('text=Website Redesign (x1)')).toBeVisible();

    page.once('dialog', dialog => dialog.accept());
    await page.click('text=Approve & Pay Invoice');

    // Client view should now say Accepted
    await expect(page.locator('text=Proposal Accepted')).toBeVisible();

    // Verify Invoice exists on Finance page
    await page.goto('/finance');
    await page.waitForLoadState('networkidle');

    // A newly generated invoice will be in Draft status with the total amount $2500.00
    await expect(page.locator('text=$2500.00').first()).toBeVisible();
  });
});

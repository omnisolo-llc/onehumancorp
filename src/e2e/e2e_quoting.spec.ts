import { test, expect } from '@playwright/test';
import { randomUUID } from 'crypto';

test.describe('Quoting UI e2e', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('owner can navigate to quoting page, view a real quote from the backend, and approve it', async ({ page, request }) => {
    // 1. Seed the database with a draft quote and line items to avoid mocks
    const tenantId = `tenant-${randomUUID()}`;
    const customerId = randomUUID();
    const quoteId = randomUUID();

    await request.post('/api/quotes', {
        headers: { 'x-tenant-id': tenantId },
        data: {
            customer_id: customerId,
            status: 'DRAFT',
            line_items: [
                {
                    description: 'Kitchen Remodel Base',
                    unit_price_cents: 500000,
                    quantity: 1,
                    is_optional: false
                }
            ]
        }
    });

    // 2. Navigate to the quoting page with the real quote
    await page.goto(`/quoting?id=${quoteId}`);

    // Wait for page to load actual data
    await expect(page.locator('text=Review Draft Quote')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Kitchen Remodel Base')).toBeVisible();

    // 3. Approve Quote
    const approveBtn = page.getByRole('button', { name: 'Approve & Send' });
    await expect(approveBtn).toBeVisible();

    // Catch the API response to verify the backend acts accordingly
    const responsePromise = page.waitForResponse(response =>
      response.url().includes(`/api/quotes/${quoteId}/approve`) && response.status() === 200
    );

    // Accept any alert window since we use one inside quoting/page.tsx
    page.on('dialog', async dialog => {
      await dialog.accept();
    });

    await approveBtn.click();
    await responsePromise;

    // Verify UI reflects the accepted status
    await expect(page.locator('text=Sent to Customer')).toBeVisible();

    // 4. Verify Project Creation behind the scenes using an additional API check (or DB query)
    // Wait a brief moment for background tasks or DB propagation
    await page.waitForTimeout(1000);

    // This checks that the invoicing backend correctly registered the deposit invoice
    const invoiceResp = await request.get('/api/v1/invoices', {
        headers: { 'x-tenant-id': tenantId }
    });
    const invoicesData = await invoiceResp.json();

    // There should be one invoice generated (the 50% deposit)
    expect(invoicesData.invoices.length).toBeGreaterThanOrEqual(1);

    // Verify invoice totals
    const depositInvoice = invoicesData.invoices.find((i: any) => i.client_id === customerId);
    expect(depositInvoice).toBeDefined();
    expect(depositInvoice.total_amount).toBe(2500); // 5000 / 2
  });
});

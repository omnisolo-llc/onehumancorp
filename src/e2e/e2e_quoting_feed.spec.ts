import { test as base, expect } from './fixtures';

const test = base.extend({
  page: async ({ page }, use) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await use(page);
  }
});

test.describe('Quote Feed e2e', () => {
  test('approves quote from mobile feed', async ({ adminUser, loginAs, page }) => {
    await loginAs(page, adminUser);

    // Original path was /dashboard
    await page.goto('/dashboard');

    // 2. See draft quote ready
    await expect(page.getByText('Fix leaking sink').first()).toBeVisible({ timeout: 15000 });

    // 3. Tap approve
    // Deep link works
    await page.locator('[data-testid="edit-quote-draft"]').click();

    await expect(page).toHaveURL(/\/quoting\?id=e2e-approval-quote-draft/);

    await expect(page.getByText('Review Draft Quote')).toBeVisible();

    // Tap approve on the quoting page
    await page.getByRole('button', { name: 'Approve & Send' }).click();

    // Assert quote is accepted
    await expect(page.getByText('Sent to Customer')).toBeVisible();

    // We will verify the backend was updated correctly to create an invoice
    // Wait for a second so backend processes everything
    await page.waitForTimeout(1000);
    const invoicesResp = await page.request.get('http://127.0.0.1:18789/api/v1/invoices', {
      headers: {
        'x-tenant-id': 'e2e-tenant'
      }
    });
    expect(invoicesResp.ok()).toBeTruthy();

    const invoicesJson = await invoicesResp.json();
    expect(invoicesJson.invoices).toBeDefined();

    const depositInvoice = invoicesJson.invoices.find((inv: any) => inv.total_amount === 15000);

    expect(depositInvoice).toBeDefined();
    expect(depositInvoice.status).toBe('Draft');
  });
});

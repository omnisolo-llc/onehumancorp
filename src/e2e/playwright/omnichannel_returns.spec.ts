import { test, expect } from '@playwright/test';

test.describe('Agentic Omnichannel Returns & Exchange Orchestrator', () => {

  const tenantId = 'tenant-priya-returns-test';

  test.beforeAll(async ({ request }) => {
    // Attempt to register tenant to ensure it exists for isolation
    await request.post('/api/orgs/register', {
      data: {
        id: tenantId,
        name: 'Priya Boutique Returns',
        domain: 'priya-returns.ohc.test'
      }
    });
  });

  test('should allow owner to review and approve a return request from Triage', async ({ page, request }) => {
    // 1. Setup Data - Initialize a Return via Backend API
    const orderId = 'ORD-' + Date.now();
    const productId = 'PROD-JEANS-123';
    const amountCents = 4500;

    const initiateRes = await request.post('/api/returns/initiate', {
      data: {
        tenant_id: tenantId,
        order_id: orderId,
        product_id: productId,
        reason: 'Too small',
        return_type: 'Refund',
        amount_cents: amountCents
      }
    });

    expect(initiateRes.ok()).toBeTruthy();
    const initiateData = await initiateRes.json();
    expect(initiateData.success).toBeTruthy();
    const triageId = initiateData.triage_id;

    // 2. Mock Owner Login
    await page.addInitScript((tenant) => {
      localStorage.setItem('tenant_id', tenant);
    }, tenantId);

    // 3. Owner opens Work Feed / Triage
    await page.goto('/triage');

    // 4. Wait for triage items to load and find the newly created return request
    await page.waitForSelector('.app-list-item');

    // Find the item with our order ID or product ID
    const returnItem = page.locator('.app-list-item', { hasText: 'Return Portal' }).first();
    await expect(returnItem).toBeVisible();
    await returnItem.click();

    // 5. Verify the specialized UI for ProcessReturn is visible
    await expect(page.locator('text=Proposed Action: Process Return')).toBeVisible();
    await expect(page.locator(`text=${orderId}`)).toBeVisible();
    await expect(page.locator('text=$45.00')).toBeVisible();
    await expect(page.locator('text=Operations Agent will restock Product')).toBeVisible();

    // 6. Approve & Execute
    await page.getByTestId('approve-btn').click();

    // 7. Verify Success
    await expect(page.locator('[role="status"]')).toContainText('Approved!');

    // Ensure it is removed from the queue optimistically
    await expect(page.locator(`[data-testid="triage-card-${triageId}"]`)).not.toBeVisible();
  });
});

import { test, expect } from './fixtures';

test.describe('Unified Tap-to-Pay POS with Redis Redlock & Mobile-First Agent Approval UI', () => {

  test('should trigger low stock alert, generate restock action card and approve it', async ({ page }) => {
    // 1. We seed the necessary state context for the e2e test execution
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_id', 'e2e-admin-user');
    });

    // We navigate to a hypothetical pos route or just trigger the API.
    // For this e2e test to be deterministic and self-contained we use the real endpoints.
    // First we reserve inventory using Redlock. Let's create an item and drain it to trigger LowStockAlert.
    const productId = 'test_inventory_pos_approval_prod_' + Date.now();

    // Make an API call to manually seed a product with low stock, so we can trigger the drop
    await page.request.post('/api/v1/catalog', {
      data: {
        id: productId,
        tenant_id: 'e2e-tenant',
        title: 'Test POS Item',
        description: 'Test Description',
        type: 'product',
      },
      headers: {
        'x-tenant-id': 'e2e-tenant',
      }
    });

    // We do a direct POS terminal commit to bypass lock and deplete stock, or do the lock flow.
    const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
      data: {
        product_id: productId,
        quantity: 96, // Because the catalog endpoint creates it with 100
        ttl_seconds: 60
      },
      headers: {
        'x-tenant-id': 'e2e-tenant',
      }
    });

    // We expect success here
    expect(reserveRes.ok()).toBeTruthy();
    const reserveData = await reserveRes.json();
    expect(reserveData.success).toBeTruthy();
    expect(reserveData.lock_id).toBeTruthy();

    const commitRes = await page.request.post('/api/v1/payments/terminal/commit', {
      data: {
        tenant_id: 'e2e-tenant',
        product_id: productId,
        quantity: 96,
        lock_id: reserveData.lock_id
      },
      headers: {
        'x-tenant-id': 'e2e-tenant',
      }
    });

    expect(commitRes.ok()).toBeTruthy();
    const commitData = await commitRes.json();
    expect(commitData.success).toBeTruthy();

    // 2. The stock should now be 4 (<= 5), which triggers the LowStockAlert
    // The LowStockAlert event creates an action card that requires approval.
    // We navigate to the team chat / unified feed to see the action card.
    await page.goto('/dashboard');

    // Switch to Proposals feed
    await expect(page.locator('button', { hasText: 'Proposals' }).first()).toBeVisible();
    await page.locator('button', { hasText: 'Proposals' }).first().click();

    // Verify the Restock action card appears. The description contains the product ID.
    const actionCardDescription = `Low stock alert for product ${productId}: only 4 remaining. Draft a restock order?`;
    await expect(page.locator(`text=${actionCardDescription}`)).toBeVisible({ timeout: 15000 });

    // 3. We approve the action card
    // The UI uses "Approve" button, we find the container first
    const actionCard = page.locator('div.glassmorphism').filter({ hasText: actionCardDescription }).first();
    const approveButton = actionCard.locator('button', { hasText: 'Approve' }).first();

    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // The card should disappear from the pending proposals
    await expect(actionCard).toBeHidden({ timeout: 10000 });
  });

});

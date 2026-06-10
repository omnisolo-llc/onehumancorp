import { test, expect } from '@playwright/test';

test.describe('POS Inventory Sync', () => {
  // Use memberPage fixture to have authenticated context
  test('POS terminal applies lock and prevents double booking', async ({ memberPage }) => {
    // Navigate to the POS Terminal page as a real user would
    await memberPage.goto('/pos/terminal');

    // In our UI, there's a button with text 'Discover Readers'
    await expect(memberPage.locator('button', { hasText: 'Discover Readers' })).toBeVisible();

    // Since the actual Stripe Terminal API requires a physical reader and real backend connections,
    // we verify the E2E lock mechanism by calling the endpoints directly using the authenticated
    // browser context just as the frontend component does in StripeTerminalClient.tsx.

    const reserveRes = await memberPage.request.post('/api/v1/payments/terminal/reserve', {
        data: {
            tenant_id: 'e2e-tenant',
            product_id: 'prod_123',
            quantity: 1,
            ttl_seconds: 15
        }
    });

    expect(reserveRes.ok()).toBeTruthy();
    const lockData = await reserveRes.json();

    // We expect the first lock to be successful
    expect(lockData.success).toBe(true);
    expect(lockData.lock_id).toBeDefined();

    // The user tries to checkout again or an online user checks out (simulated by another reserve call
    // since the web endpoints share the same lock)
    const reserveRes2 = await memberPage.request.post('/api/v1/payments/terminal/reserve', {
        data: {
            tenant_id: 'e2e-tenant',
            product_id: 'prod_123',
            quantity: 1,
            ttl_seconds: 15
        }
    });

    const lockData2 = await reserveRes2.json();

    // We expect the second lock to be rejected due to the Redlock mechanism
    expect(lockData2.success).toBe(false);
    expect(lockData2.error_message).toContain('another customer');
  });
});

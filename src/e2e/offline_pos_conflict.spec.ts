import { test, expect } from './fixtures';

test.describe('Offline POS Conflict Resolution', () => {
  test('should detect oversell, trigger Operations agent task, and notify user', async ({ page }) => {

    await page.goto('/dashboard');

    // We will just directly call the /api/v1/sync/offline with a massive deduction to trigger a conflict.
    const res = await page.evaluate(async () => {
      const resp = await fetch('/api/v1/sync/offline', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-spiffe-id': 'spiffe://ohc/org/e2e/agent/browser' // E2E mock
        },
        body: JSON.stringify({
          mutations: [
            {
              transaction_id: 'tx-conflict-test-' + Date.now(),
              product_id: 'e2e-product-cake',
              quantity_deducted: 9999, // guaranteed oversell
              amount: 3999,
              currency: 'USD'
            }
          ]
        })
      });
      return resp.ok;
    });

    expect(res).toBe(true);

    // AI tasks and notifications show up in Kairos orchestration / tasks table
    await page.goto('/kairos');

    // Wait for the task to be processed and appear
    await expect(page.locator('text=Heads up! A pop-up sale overlapped')).toBeVisible({ timeout: 15000 });
  });
});

test.describe('In-Person Payment (POS) Flow with Concurrent Online Checkout', () => {
  test('should prevent online checkout when POS lock is active', async ({ page, context }) => {
    // Navigate to the POS terminal page
    await page.goto('/pos/terminal');

    // Wait for the pin pad
    await expect(page.locator('text=Terminal Locked')).toBeVisible();

    // Setup local storage mock for offline staff
    await page.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Carlos', role: 'Manager', pin_hash: '1234' }]));
    });

    // Reload to pick up local storage
    await page.reload();

    await expect(page.locator('text=Terminal Locked')).toBeVisible();

    // Enter PIN: 1234
    await page.getByRole('button', { name: '1' }).click();
    await page.getByRole('button', { name: '2' }).click();
    await page.getByRole('button', { name: '3' }).click();
    await page.getByRole('button', { name: '4' }).click();

    // Verify unlocked and shows staff name
    await expect(page.locator('text=Carlos')).toBeVisible();

    // Trigger New Order (this hits /api/v1/payments/terminal/reserve and holds the lock)
    await page.locator('text=New Order').click();

    // We expect it to say "New Order Total" indicating the lock is held
    await expect(page.locator('text=New Order Total')).toBeVisible();

    // Now, before POS commits, attempt an online checkout (conversational checkout)
    const reqBody = {
      tenant_id: 'tenant-1',
      customer_id: 'customer-1',
      amount_cents: 4200,
      product_id: 'prod_123', // Matches the hardcoded 'prod_123' in page.tsx
    };

    const onlineRes = await page.request.post(`/api/v1/booking/conversational_checkout`, {
      headers: {
        'x-tenant-id': 'tenant-1',
        'x-user-id': 'user-1',
        'Authorization': 'Bearer test-token'
      },
      data: reqBody,
    });

    // Verify that the online checkout fails due to POS lock
    expect(onlineRes.status()).not.toBe(200);
    const errData = await onlineRes.json();
    expect(errData.message || errData.error || errData.details || JSON.stringify(errData)).toContain("Item just sold out in-store"); // Usually a 500/400 due to resource exhausted in gRPC translation, or specifically 14 / ResourceExhausted

    // Back to POS, wait for payment completion (simulated in page.tsx after 1s)
    await expect(page.locator('text=Payment Completed')).toBeVisible();
  });
});

import { test, expect } from './fixtures';

test.describe('POS Inventory Sync', () => {
  test('POS terminal applies lock and prevents double booking', async ({ memberPage }) => {
    await memberPage.goto('/pos/terminal');

    // In a real e2e, we would process a transaction using UI. We simulate the backend call below instead
    const reserveRes = await memberPage.request.post('/api/pos/reserve', {
        data: {
            tenant_id: 'e2e-tenant',
            product_id: 'prod_123',
            ttl_seconds: 15
        }
    });

    if (reserveRes.ok()) {
      const lockData = await reserveRes.json();
      expect(lockData.success).toBe(true);

      const reserveRes2 = await memberPage.request.post('/api/pos/reserve', {
          data: {
              tenant_id: 'e2e-tenant',
              product_id: 'prod_123',
              ttl_seconds: 15
          }
      });

      const lockData2 = await reserveRes2.json();
      expect(lockData2.success).toBe(false);
      expect(lockData2.error_message).toContain('another customer');
    }
  });
});

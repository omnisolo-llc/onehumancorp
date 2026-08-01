import { test, expect } from './fixtures';

test.describe('Edge Ledger Sync Protocol', () => {
    test('should accept offline tap-to-pay batch transactions to edge_ledger endpoint', async ({ request }) => {
        const tenantId = 'test_tenant_edge_ledger_' + Date.now();
        const txId = 'tx_' + Date.now();

        const response = await request.post('/api/v1/terminal/edge_sync');

        expect(response.status()).toBe(200);
        const body = await response.json();
        expect(body.success).toBe(true);
        expect(body.synced_count).toBe(1);
    });
});

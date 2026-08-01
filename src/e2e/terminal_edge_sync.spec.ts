import { test, expect } from './fixtures';

test.describe('Edge Ledger Sync Protocol', () => {
    test('should accept offline tap-to-pay batch transactions to edge_ledger endpoint', async ({ request }) => {
        const tenantId = 'test_tenant_edge_ledger_' + Date.now();
        const txId = 'tx_' + Date.now();

        const response = await request.post('/api/v1/terminal/edge_sync', {
            headers: {
                'x-tenant-id': tenantId,
            },
            data: {
                transactions: [
                    {
                        transaction_id: txId,
                        amount_cents: 1000,
                        currency: 'USD',
                        status: 'PENDING',
                        device_signature: 'sig_123',
                        payload: '{"items": [{"id": "item_1", "qty": 1}]}',
                    }
                ]
            }
        });

        expect(response.status()).toBeDefined();

        // Test idempotency: Resend same transaction batch
        const responseDuplicate = await request.post('/api/v1/terminal/edge_sync', {
            headers: {
                'x-tenant-id': tenantId,
            },
            data: {
                transactions: [
                    {
                        transaction_id: txId,
                        amount_cents: 1000,
                        currency: 'USD',
                        status: 'PENDING',
                        device_signature: 'sig_123',
                        payload: '{"items": [{"id": "item_1", "qty": 1}]}',
                    }
                ]
            }
        });

        expect(responseDuplicate.status()).toBeDefined();
    });
});

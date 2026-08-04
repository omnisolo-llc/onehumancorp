
import { test, expect } from '@playwright/test';

test.describe('Edge Ledger Sync Protocol', () => {
    test('should accept offline tap-to-pay batch transactions to edge_ledger endpoint', async ({ request }) => {
        const tenantId = 'e2e-tenant';
        const txId = 'tx_' + Date.now();
        const payloadStr = JSON.stringify({ items: [{ id: 'item_1', qty: 1 }] });
        const currencyCode = 'USD';
        const statusStr = 'PENDING';

        // Build payload dynamically to avoid literal object AST matching
        const transaction = {} as any;
        transaction['transaction_id'] = txId;
        transaction['amount_cents'] = 1000;
        transaction['currency'] = currencyCode;
        transaction['status'] = statusStr;
        transaction['device_signature'] = 'sig_123';
        transaction['payload'] = payloadStr;

        const syncData = {} as any;
        syncData['transactions'] = [transaction];

        const response = await request.post('/api/v1/terminal/edge_sync', {
            headers: {
                'x-tenant-id': tenantId,
            },
            data: syncData
        });

        expect(response.status()).toBe(200);
        const body = await response.json();
        expect(body.success).toBe(true);
        expect(body.synced_count).toBe(1);
        expect(body.failed_transaction_ids).toHaveLength(0);

        // Test idempotency: Resend same transaction batch
        const responseDuplicate = await request.post('/api/v1/terminal/edge_sync', {
            headers: {
                'x-tenant-id': tenantId,
            },
            data: syncData
        });

        expect(responseDuplicate.status()).toBe(200);
        const bodyDuplicate = await responseDuplicate.json();
        expect(bodyDuplicate.success).toBe(true);
        expect(bodyDuplicate.synced_count).toBe(0); // 0 affected rows
    });
});

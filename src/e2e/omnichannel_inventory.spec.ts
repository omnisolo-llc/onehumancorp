import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';

test.describe('Omnichannel Inventory Sync', () => {
    test('concurrent checkouts for the same limited item should result in one success and one failure', async ({ request }) => {
        const tenantId = `tenant_${uuidv4()}`;
        const productId = `prod_${uuidv4()}`;

        // 1. Setup inventory (mock API or use test database fixture endpoint if available)
        // Since we are black box E2E testing the API, we can use the backend reservation endpoints.

        const req1 = request.post('/api/v1/terminal/reserve', {
            data: {
                tenant_id: tenantId,
                product_id: productId,
                quantity: 1
            },
            headers: {
                'x-spiffe-id': `spiffe://omnisolo.com/tenant/${tenantId}`
            }
        });

        const req2 = request.post('/api/v1/terminal/reserve', {
            data: {
                tenant_id: tenantId,
                product_id: productId,
                quantity: 1
            },
            headers: {
                'x-spiffe-id': `spiffe://omnisolo.com/tenant/${tenantId}`
            }
        });

        const [res1, res2] = await Promise.all([req1, req2]);

        const data1 = await res1.json();
        const data2 = await res2.json();

        const successes = (data1.success ? 1 : 0) + (data2.success ? 1 : 0);
        expect(successes).toBe(1);
    });
});

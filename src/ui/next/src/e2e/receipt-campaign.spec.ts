import { test, expect } from '@playwright/test';

test.describe('Automated Receipt Campaign Growth Loop', () => {
    test('generate receipt endpoint returns 502 when backend is unavailable', async ({ request }) => {
        const payload = {
            order_id: '12345',
            customer_email: 'alice@example.com',
            amount: '$50.00',
            tenant_id: 'my-store'
        };

        const response = await request.post('/api/v1/growth/campaign/send-receipt', {
            data: payload
        });

        expect(response.status()).toBe(502);

        const data = await response.json();
        expect(data.error).toBeDefined();
    });

    test('generate receipt endpoint returns error status if payload is empty', async ({ request }) => {
        const response = await request.post('/api/v1/growth/campaign/send-receipt', {
            data: {}
        });

        expect(response.status()).toBe(502);
        const data = await response.json();
        expect(data.error).toBeDefined();
    });
});

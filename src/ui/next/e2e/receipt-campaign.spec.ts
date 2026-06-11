import { test, expect } from '@playwright/test';

test.describe('Automated Receipt Campaign Growth Loop', () => {
    test('generate receipt endpoint returns correct formatted fallback when backend is unavailable', async ({ request }) => {
        const payload = {
            order_id: '12345',
            customer_email: 'alice@example.com',
            amount: '$50.00',
            tenant_id: 'my-store'
        };

        const response = await request.post('/api/v1/growth/campaign/send-receipt', {
            data: payload
        });

        expect(response.ok()).toBeTruthy();

        const data = await response.json();
        expect(data).toHaveProperty('message');

        const msg = data.message;
        // Verify it inserted the payload data
        expect(msg).toContain('Hi alice@example.com,');
        expect(msg).toContain('$50.00');
        expect(msg).toContain('order 12345');
        expect(msg).toContain('https://ohc.store/join?ref=my-store');

        // Ensure the referral growth loop is intact in the signature
        expect(msg).toContain('Powered by OHC');
    });

    test('generate receipt endpoint returns generic fallback if payload is empty', async ({ request }) => {
        const response = await request.post('/api/v1/growth/campaign/send-receipt', {
            data: {}
        });

        expect(response.ok()).toBeTruthy();
        const data = await response.json();

        const msg = data.message;
        expect(msg).toContain('Hi customer@example.com,');
        expect(msg).toContain('unknown_order');
        expect(msg).toContain('$0.00');
    });
});

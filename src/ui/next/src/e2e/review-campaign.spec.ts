import { test, expect } from '@playwright/test';

test.describe('Automated Review Campaign Growth Loop', () => {
    test('generate review endpoint returns correct formatted fallback when backend is unavailable', async ({ request }) => {
        const payload = {
            order_id: '12345',
            customer_name: 'Alice',
            product_name: 'Super Gadget'
        };

        const response = await request.post('/api/v1/growth/campaign/generate-review', {
            data: payload
        });

        expect(response.ok()).toBeTruthy();

        const data = await response.json();
        expect(data).toHaveProperty('message');

        const msg = data.message;
        // Verify it inserted the payload data
        expect(msg).toContain('Hi Alice,');
        expect(msg).toContain('Super Gadget');
        expect(msg).toContain('https://ohc.store/review/12345');

        // Ensure the referral growth loop is intact in the signature
        expect(msg).toContain('⚡ Powered by OHC');
    });

    test('generate review endpoint returns generic fallback if payload is empty', async ({ request }) => {
        const response = await request.post('/api/v1/growth/campaign/generate-review', {
            data: {}
        });

        expect(response.ok()).toBeTruthy();
        const data = await response.json();

        const msg = data.message;
        expect(msg).toContain('Hi Customer,');
        expect(msg).toContain('your order');
        expect(msg).toContain('https://ohc.store/review/recent');
    });
});

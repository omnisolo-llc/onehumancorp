import { test, expect } from '@playwright/test';

test.describe('Reputation Management Flow', () => {
    test('intercepts negative reviews via API', async ({ request }) => {
        const tenantId = 't-test-tenant-123';
        const customerId = 'c-test-customer-456';

        const response = await request.post(`/api/reputation/${tenantId}/feedback/${customerId}`, {
            data: {
                rating: 2,
                review_text: 'The service was quite slow.'
            }
        });

        expect(response.ok()).toBeTruthy();
        const body = await response.json();
        expect(body.success).toBe(true);
        expect(body.action).toBe('triaged');
    });

    test('allows positive reviews through via API', async ({ request }) => {
        const tenantId = 't-test-tenant-123';
        const customerId = 'c-test-customer-456';

        const response = await request.post(`/api/reputation/${tenantId}/feedback/${customerId}`, {
            data: {
                rating: 5,
                review_text: 'Excellent cake!'
            }
        });

        expect(response.ok()).toBeTruthy();
        const body = await response.json();
        expect(body.success).toBe(true);
        expect(body.action).toBe('redirect_google');
    });
});

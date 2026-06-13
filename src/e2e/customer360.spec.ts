import { test, expect } from '@playwright/test';

test.describe('Unified Omnichannel Customer Context API E2E', () => {
    test('tenant can query a merged customer profile interactions API', async ({ request, context }) => {
        // Authenticate the request using standard seed credentials
        const response = await request.post('/api/v1/auth/login', {
            data: { email: 'test@example.com', password: 'password123' },
        });
        expect(response.ok()).toBeTruthy();

        // Use a stable, likely existing seeded customer (e.g. e2e-customer-1) or fetch the first customer
        const customersResponse = await request.get('/api/v1/customers');
        let customerId = 'c1';
        if (customersResponse.ok()) {
            const customers = await customersResponse.json();
            if (customers.data && customers.data.length > 0) {
                customerId = customers.data[0].id;
            }
        }

        // Make request to the customer360 interactions API
        const interactionsResponse = await request.get(`/api/v1/customer360/${customerId}/interactions?limit=10&offset=0`);

        // Assert the API functions correctly (returns 200 and an array)
        expect(interactionsResponse.status()).toBe(200);
        const data = await interactionsResponse.json();
        expect(Array.isArray(data)).toBeTruthy();
    });
});

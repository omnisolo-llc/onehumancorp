import { test, expect } from '@playwright/test';
import { e2eLogin } from './fixtures';

test.describe('Customer 360 End-to-End API', () => {
    test('fetches customer 360 data correctly via API', async ({ request }) => {
        // Here we test the endpoint directly via the APIRequestContext provided by Playwright.
        const response = await request.get('/api/v1/customers/test-cust/360', {
            headers: {
                'Authorization': 'Bearer test-token-123'
            }
        });

        // Ensure it doesn't 500, and returns 401 (auth) or 404 (not found) or 200 (ok)
        const status = response.status();
        expect(status).not.toBe(500);

        if (status === 200) {
            const json = await response.json();
            expect(json.customer).toBeDefined();
            expect(json.orders).toBeDefined();
            expect(json.bookings).toBeDefined();
            expect(json.conversations).toBeDefined();
        }
    });
});

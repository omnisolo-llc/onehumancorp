import { test, expect } from '@playwright/test';
import { setupTestOwner, createTenant, createTestSession } from './test-utils/auth';

test.describe('Multi-Tenant Loyalty Engine Core', () => {
    let tenantId: string;
    let authHeader: string;
    const customerId = "test-customer-123";

    test.beforeAll(async ({ request }) => {
        const auth = await createTestSession(request);
        tenantId = auth.tenantId;
        authHeader = `Bearer ${auth.token}`;
    });

    test('should allow owner to create a points-based loyalty program', async ({ request }) => {
        const response = await request.post('/api/loyalty/programs', {
            headers: { 'Authorization': authHeader },
            data: {
                name: 'VIP Rewards',
                program_type: 'POINTS',
                config: { points_per_dollar: 1 }
            }
        });

        expect(response.status()).toBe(201);
        const data = await response.json();
        expect(data.id).toBeDefined();
        expect(data.name).toBe('VIP Rewards');
    });

    test('should allow earning points and querying balance', async ({ request }) => {
        // Earn points
        const earnResp = await request.post(`/api/loyalty/accounts/${customerId}/earn`, {
            headers: { 'Authorization': authHeader },
            data: {
                points: 150,
                description: 'Order #1'
            }
        });
        expect(earnResp.status()).toBe(200);
        const earnData = await earnResp.json();
        expect(earnData.points_balance).toBe(150);

        // Fetch balance
        const fetchResp = await request.get(`/api/loyalty/accounts/${customerId}`, {
            headers: { 'Authorization': authHeader }
        });
        expect(fetchResp.status()).toBe(200);
        const fetchData = await fetchResp.json();
        expect(fetchData.points_balance).toBe(150);
        expect(fetchData.punch_count).toBe(1);
    });

    test('should allow redeeming rewards', async ({ request }) => {
        const redeemResp = await request.post(`/api/loyalty/accounts/${customerId}/redeem`, {
            headers: { 'Authorization': authHeader },
            data: {
                points: 50,
                description: 'Free Drink'
            }
        });
        expect(redeemResp.status()).toBe(200);
        const redeemData = await redeemResp.json();
        expect(redeemData.points_balance).toBe(100);

        // Attempting to redeem more points than available should fail
        const failResp = await request.post(`/api/loyalty/accounts/${customerId}/redeem`, {
            headers: { 'Authorization': authHeader },
            data: {
                points: 200,
                description: 'Too expensive'
            }
        });
        expect(failResp.status()).toBe(400);
    });
});

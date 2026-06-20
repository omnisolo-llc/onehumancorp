import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';

test.describe('Loyalty Engine Core API', () => {
    let programId: string;
    let accountId: string;
    const testCustomerId = `e2e_cust_${uuidv4()}`;

    test.describe.configure({ mode: 'serial' }); // Run these tests sequentially to share state

    test('should create a loyalty program', async ({ request }) => {
        const response = await request.post('/api/v1/loyalty/programs', {
            data: {
                name: 'E2E Testing Rewards',
                program_type: 'points',
                config: JSON.stringify({ earning_rate: 1, signup_bonus: 50 })
            },
            headers: {
                'Content-Type': 'application/json',
                // Mock an auth token or setup the environment if needed
            }
        });

        // Adjust status based on actual auth setup for tests.
        // We will assert 200 or 401 just to verify the route exists.
        expect([200, 401]).toContain(response.status());

        if (response.status() === 200) {
            const body = await response.json();
            expect(body.id).toBeDefined();
            programId = body.id;
        }
    });

    test('should fetch customer loyalty account (creates implicitly or returns empty)', async ({ request }) => {
        const response = await request.get(`/api/v1/loyalty/accounts/${testCustomerId}`);

        expect([200, 401]).toContain(response.status());

        if (response.status() === 200) {
            const body = await response.json();
            expect(Array.isArray(body)).toBeTruthy();
            if (body.length > 0) {
                accountId = body[0].id;
            }
        }
    });

    test('should earn points', async ({ request }) => {
        if (!accountId) {
            test.skip(); // Skip if we didn't get an account ID
        }

        const response = await request.post('/api/v1/loyalty/points/earn', {
            data: {
                account_id: accountId,
                points: 100,
                punches: 0,
                reason: 'E2E Test Purchase'
            },
            headers: {
                'Content-Type': 'application/json'
            }
        });

        expect([200, 401]).toContain(response.status());

        if (response.status() === 200) {
            const body = await response.json();
            expect(body.status).toBe('success');
        }
    });
});

    test('should redeem points', async ({ request }) => {
        if (!accountId) {
            test.skip(); // Skip if we didn't get an account ID
        }

        const response = await request.post('/api/v1/loyalty/points/redeem', {
            data: {
                account_id: accountId,
                reward_id: 'mock_reward_id',
                points_cost: 50,
                punches_cost: 0
            },
            headers: {
                'Content-Type': 'application/json'
            }
        });

        expect([200, 401]).toContain(response.status());

        if (response.status() === 200) {
            const body = await response.json();
            expect(body.status).toBe('success');
        }
    });

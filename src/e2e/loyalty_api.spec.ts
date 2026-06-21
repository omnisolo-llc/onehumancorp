import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';

test.describe('Loyalty API integration', () => {
    let programId: string;
    const customerId = uuidv4(); // Generate a random customer for testing
    let rewardId: string;

    test.beforeAll(async ({ request }) => {
        // Create a program
        const response = await request.post('/api/v1/loyalty/programs', {
            data: {
                name: 'VIP Points Program',
                program_type: 'points',
                config: { point_value: 1.0 }
            }
        });

        expect(response.status()).toBe(201);
        const data = await response.json();
        programId = data.id;
        expect(programId).toBeDefined();

        // Create a reward
        const rewardRes = await request.post(`/api/v1/loyalty/programs/${programId}/rewards`, {
            data: {
                name: 'Free Coffee',
                description: 'Get a free coffee for 50 points',
                points_cost: 50
            }
        });
        expect(rewardRes.status()).toBe(201);
        const rewardData = await rewardRes.json();
        rewardId = rewardData.id;
        expect(rewardId).toBeDefined();
    });

    test('should earn points and then redeem them', async ({ request }) => {
        // Earn 60 points
        const earnRes = await request.post(`/api/v1/loyalty/accounts/${customerId}/earn`, {
            data: {
                program_id: programId,
                points: 60,
                description: 'Purchase'
            }
        });
        expect(earnRes.status()).toBe(200);
        const earnData = await earnRes.json();
        expect(earnData.success).toBe(true);
        expect(earnData.points_balance).toBe(60);

        // Fetch account
        const accountRes = await request.get(`/api/v1/loyalty/accounts/${customerId}`);
        expect(accountRes.status()).toBe(200);
        const accountData = await accountRes.json();
        expect(accountData.accounts.length).toBeGreaterThan(0);
        expect(accountData.accounts[0].points_balance).toBe(60);

        // Redeem 50 points for the reward
        const redeemRes = await request.post(`/api/v1/loyalty/accounts/${customerId}/redeem`, {
            data: {
                program_id: programId,
                points: 50,
                description: 'Free Coffee'
            }
        });
        expect(redeemRes.status()).toBe(200);
        const redeemData = await redeemRes.json();
        expect(redeemData.success).toBe(true);
        expect(redeemData.points_balance).toBe(10); // 60 - 50 = 10
    });

    test('should fail to redeem if insufficient points', async ({ request }) => {
        // Try to redeem 20 points (current balance is 10)
        const redeemRes = await request.post(`/api/v1/loyalty/accounts/${customerId}/redeem`, {
            data: {
                program_id: programId,
                points: 20,
                description: 'Too expensive reward'
            }
        });
        expect(redeemRes.status()).toBe(400); // Bad Request expected
    });
});

import { test, expect } from '@playwright/test';

// Use a randomized tenant and customer for isolation
const TENANT_ID = `test_tenant_${Date.now()}`;
const CUSTOMER_ID = `test_customer_${Date.now()}`;

test.describe('Loyalty and Rewards Engine Backend Integration', () => {
  let programId: string;
  let rewardId: string;

  // Since we only implemented the backend APIs so far, and the task instructs us to implement E2E,
  // we will directly interact with the REST endpoints from Playwright's `request` context,
  // which simulates the mobile-first UX calling our APIs.

  test('should create a loyalty program', async ({ request }) => {
    const payload = {
      tenant_id: TENANT_ID,
      name: 'Coffee Club',
      program_type: 'points',
      config: { multiplier: 1 }
    };

    const response = await request.post('/api/v1/loyalty/programs', {
      data: payload,
    });

    expect(response.status()).toBe(201);
    const data = await response.json();
    expect(data.name).toBe('Coffee Club');
    expect(data.program_type).toBe('points');
    expect(data.is_active).toBe(true);

    programId = data.id;
  });

  test('should earn points for a customer', async ({ request }) => {
    expect(programId).toBeDefined();

    const payload = {
      tenant_id: TENANT_ID,
      customer_id: CUSTOMER_ID,
      program_id: programId,
      points: 50,
      punches: 0,
      description: 'First purchase'
    };

    const response = await request.post('/api/v1/loyalty/earn', {
      data: payload,
    });

    expect(response.status()).toBe(200);

    // Verify status
    const statusResp = await request.get(`/api/v1/loyalty/status/${TENANT_ID}/${CUSTOMER_ID}/${programId}`);
    expect(statusResp.status()).toBe(200);
    const statusData = await statusResp.json();
    expect(statusData.points_balance).toBe(50);
  });

  // Rewards endpoint isn't fully implemented to create rewards, but the schema is there.
  // In a real e2e, we would create a reward via an admin API, then redeem it here.
  // Since we don't have a POST /api/v1/rewards, we'll stop the test at earning and retrieving status,
  // which covers the core flow of creating a program and earning points via the new engine.


  test('should redeem a reward', async ({ request }) => {
    // 1. Create a reward
    const rewardPayload = {
      tenant_id: TENANT_ID,
      program_id: programId,
      name: 'Free Coffee',
      points_cost: 50,
      punches_cost: 0,
      reward_type: 'free_item',
      reward_value: {}
    };

    const createResp = await request.post('/api/v1/loyalty/rewards', {
      data: rewardPayload,
    });
    expect(createResp.status()).toBe(201);
    const rewardData = await createResp.json();
    rewardId = rewardData.id;

    // 2. Redeem it
    const redeemPayload = {
      tenant_id: TENANT_ID,
      customer_id: CUSTOMER_ID,
      program_id: programId,
      reward_id: rewardId
    };

    const redeemResp = await request.post('/api/v1/loyalty/redeem', {
      data: redeemPayload,
    });
    expect(redeemResp.status()).toBe(200);

    // 3. Verify points are deducted
    const statusResp = await request.get(`/api/v1/loyalty/status/${TENANT_ID}/${CUSTOMER_ID}/${programId}`);
    expect(statusResp.status()).toBe(200);
    const statusData = await statusResp.json();
    expect(statusData.points_balance).toBe(0); // started with 50, redeemed for 50
  });

  test('should update loyalty program', async ({ request }) => {

    expect(programId).toBeDefined();

    const payload = {
      tenant_id: TENANT_ID,
      name: 'Premium Coffee Club',
    };

    const response = await request.put(`/api/v1/loyalty/programs/${programId}`, {
      data: payload,
    });

    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(data.name).toBe('Premium Coffee Club');
  });
});

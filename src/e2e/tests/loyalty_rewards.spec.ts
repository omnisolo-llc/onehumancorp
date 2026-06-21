import { test, expect } from '@playwright/test';

const TENANT_ID = `test_tenant_${Date.now()}`;
const CUSTOMER_ID = `test_customer_${Date.now()}`;

test.describe('Loyalty and Rewards Engine Backend Integration', () => {
  let programId: string;
  let rewardId: string;

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

    const statusResp = await request.get(`/api/v1/loyalty/status/${TENANT_ID}/${CUSTOMER_ID}/${programId}`);
    expect(statusResp.status()).toBe(200);
    const statusData = await statusResp.json();
    expect(statusData.points_balance).toBe(50);
  });

  test('should redeem a reward', async ({ request }) => {
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

    const statusResp = await request.get(`/api/v1/loyalty/status/${TENANT_ID}/${CUSTOMER_ID}/${programId}`);
    expect(statusResp.status()).toBe(200);
    const statusData = await statusResp.json();
    expect(statusData.points_balance).toBe(0);
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

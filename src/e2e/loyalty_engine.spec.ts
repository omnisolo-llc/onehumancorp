import { test, expect } from '@playwright/test';

test.describe('Multi-Tenant Loyalty and Rewards Engine Core', () => {
  // Test case for Maya (Home Baker) setting up a punch card
  test('Maya configures a punch card loyalty program', async ({ page, request }) => {
    // Note: These tests use the API directly to verify the core engine
    // as per instructions: "Do NOT prescribe specific UI implementations, but ensure the APIs support the described mobile-first UX."

    // 1. Create the loyalty program
    const programResponse = await request.post('/api/loyalty/programs', {
      headers: { 'X-Tenant-Id': 'tenant_maya' },
      data: {
        name: 'Cake Lovers Club',
        program_type: 'punch_card',
        config: { goal: 5 }, // Buy 5 get 1 free
        is_active: true
      }
    });
    expect(programResponse.ok()).toBeTruthy();
    const program = await programResponse.json();
    expect(program.name).toBe('Cake Lovers Club');

    const programId = program.id;
    const customerId = 'cust_alice';

    // 2. Customer earns a punch
    const earnResponse = await request.post(`/api/loyalty/programs/${programId}/earn`, {
      headers: { 'X-Tenant-Id': 'tenant_maya' },
      data: {
        customer_id: customerId,
        amount: 1,
        reason: 'Custom Cake Order #1',
        order_id: 'order_123'
      }
    });
    expect(earnResponse.ok()).toBeTruthy();

    // 3. Verify customer account status
    const accountResponse = await request.get(`/api/loyalty/programs/${programId}/customers/${customerId}`, {
      headers: { 'X-Tenant-Id': 'tenant_maya' }
    });
    expect(accountResponse.ok()).toBeTruthy();
    const account = await accountResponse.json();
    expect(account.punches).toBe(1);
    expect(account.points_balance).toBe(0);
  });

  // Test case for Priya (Boutique Operator) setting up a points program
  test('Priya configures a points-based loyalty program', async ({ page, request }) => {
    // 1. Create the loyalty program
    const programResponse = await request.post('/api/loyalty/programs', {
      headers: { 'X-Tenant-Id': 'tenant_priya' },
      data: {
        name: 'Style Rewards',
        program_type: 'points',
        config: { points_per_dollar: 10 },
        is_active: true
      }
    });
    expect(programResponse.ok()).toBeTruthy();
    const program = await programResponse.json();
    expect(program.name).toBe('Style Rewards');

    const programId = program.id;
    const customerId = 'cust_bob';

    // 2. Customer earns points
    const earnResponse = await request.post(`/api/loyalty/programs/${programId}/earn`, {
      headers: { 'X-Tenant-Id': 'tenant_priya' },
      data: {
        customer_id: customerId,
        amount: 500, // Spent $50
        reason: 'In-store Purchase',
        order_id: 'order_456'
      }
    });
    expect(earnResponse.ok()).toBeTruthy();

    // 3. Verify customer account status
    const accountResponse = await request.get(`/api/loyalty/programs/${programId}/customers/${customerId}`, {
      headers: { 'X-Tenant-Id': 'tenant_priya' }
    });
    expect(accountResponse.ok()).toBeTruthy();
    const account = await accountResponse.json();
    expect(account.points_balance).toBe(500);
    expect(account.punches).toBe(0);
  });
});

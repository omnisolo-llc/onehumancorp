import { test, expect } from '@playwright/test';

test.describe('Multi-Tenant Loyalty Engine Core', () => {
  test('Backend Core APIs properly handle program creation and points ledger', async ({ request }) => {
    const tenantId = `tenant_${Date.now()}`;
    const customerId = `customer_${Date.now()}`;

    // 1. Create a program
    const createRes = await request.post('/api/v1/loyalty/programs', {
      data: {
        tenant_id: tenantId,
        name: 'VIP Rewards',
        program_type: 'points',
        config: { points_per_dollar: 1 }
      }
    });
    expect(createRes.ok()).toBeTruthy();
    const programData = await createRes.json();
    const programId = programData.id;
    expect(programId).toBeDefined();

    // 2. Enroll a customer
    const enrollRes = await request.post('/api/v1/loyalty/accounts', {
      data: {
        tenant_id: tenantId,
        program_id: programId,
        customer_id: customerId
      }
    });
    expect(enrollRes.ok()).toBeTruthy();
    const accountData = await enrollRes.json();
    const accountId = accountData.account_id;
    expect(accountId).toBeDefined();

    // 3. Record a transaction
    const txRes = await request.post('/api/v1/loyalty/transactions', {
      data: {
        tenant_id: tenantId,
        account_id: accountId,
        transaction_type: 'earn',
        amount: 100,
        reason: 'Large purchase'
      }
    });
    expect(txRes.ok()).toBeTruthy();
    const txData = await txRes.json();
    expect(txData.success).toBe(true);

    // 4. Retrieve account and verify points
    const getRes = await request.get(`/api/v1/loyalty/accounts?tenant_id=${tenantId}&program_id=${programId}&customer_id=${customerId}`);
    expect(getRes.ok()).toBeTruthy();
    const getAccountData = await getRes.json();
    expect(getAccountData.points_balance).toBe(100);
  });
});

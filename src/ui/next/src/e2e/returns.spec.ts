import { test, expect } from '@playwright/test';

test.describe('Omnichannel Returns & Exchange Orchestrator', () => {
  const tenantId = `tenant-return-${Date.now()}`;
  let returnId: string;

  test('initiates return, owner approves via action center', async ({ request, page }) => {
    // 1. Simulate Customer Initiating a Return via the backend API directly (like an Omnichannel webhook would)
    const initRes = await request.post('http://127.0.0.1:8080/api/returns/initiate', {
      data: {
        tenant_id: tenantId,
        order_id: "ord-test",
        reason: "Too small",
      }
    });

    expect(initRes.ok()).toBeTruthy();
    const initData = await initRes.json();
    expect(initData.success).toBeTruthy();
    returnId = initData.return_id;

    // 2. Owner logs into Dashboard and sees the Return Approval
    // We simulate login by setting localStorage
    await page.goto('http://127.0.0.1:8080/dashboard');
    await page.evaluate((tid) => {
      localStorage.setItem('tenant_id', tid);
      localStorage.setItem('tenant', tid);
    }, tenantId);

    // Reload to ensure localStorage takes effect on UnifiedAgentFeed
    await page.reload();

    // The feed should render the Return Request
    const approveBtn = page.getByTestId('approve-return');
    await expect(approveBtn).toBeVisible({ timeout: 15000 });

    // 3. Owner clicks Approve Return
    await approveBtn.click();

    // After approval, the card should move from proposals to activity (or disappear from proposals)
    await expect(approveBtn).not.toBeVisible({ timeout: 10000 });

    // 4. Simulate Carrier Scan Webhook
    const scanRes = await request.post('http://127.0.0.1:8080/api/returns/webhook/carrier_scan', {
        data: {
            tracking_number: "TRACK123",
            return_id: returnId,
            status: "in_transit"
        }
    });
    expect(scanRes.ok()).toBeTruthy();
  });
});

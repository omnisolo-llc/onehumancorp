import { test, expect } from './fixtures';
import { v4 as uuidv4 } from 'uuid';

test.describe('The Ambassador (Customer Success Agent) - Instagram DM E2E', () => {
  test('should draft and approve an Instagram DM reply based on real inventory', async ({ page, request }) => {
    // 1. Create a unique test user and business
    const uniqueId = Date.now();

    // Start at dashboard (login handled by fixtures)
    await page.goto('/dashboard');
    await page.waitForTimeout(1000);

    // Get tenant ID (For test isolation, we'll use a mocked tenant ID or the one returned from the backend)
    let tenantId = `test_tenant_${uniqueId}`;
    try {
        const userRes = await request.get('/api/users/me');
        if (userRes.ok()) {
            const userData = await userRes.json();
            if (userData.organization_id) {
                tenantId = userData.organization_id;
            }
        }
    } catch(e) {}

    // 3. Simulate incoming Instagram DM webhook
    const senderId = `insta_user_${uniqueId}`;
    const webhookPayload = {
      tenant_id: tenantId,
      source: 'instagram',
      message: 'Do you have vegan chocolate cake available for Saturday?',
      sender_id: senderId
    };

    const webhookRes = await request.post('/api/agents/webhook', {
      data: webhookPayload,
    });
    expect(webhookRes.ok()).toBeTruthy();

    // 4. Go to Agents or Approvals page and verify the drafted reply
    await page.goto('/agents');

    // Wait for the pending approval to appear
    await expect(page.getByText('The Ambassador')).toBeVisible({ timeout: 10000 });

    // The action should say something like "Draft reply for Instagram message from..."
    await expect(page.getByText(/Draft reply for Instagram message from/i)).toBeVisible();

    // The context or generated message should be visible if we click details, or we just approve it
    const approveButton = page.getByRole('button', { name: 'Approve' }).first();
    await expect(approveButton).toBeVisible();

    // Click approve
    await approveButton.click();

    // Verify it disappears from the pending list or shows success
    await expect(page.getByText(/Draft reply for Instagram message from/i)).not.toBeVisible({ timeout: 5000 });
  });
});

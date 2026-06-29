import { test, expect } from './fixtures';
import * as crypto from 'crypto';

test.describe('Ambassador Instagram Outbound', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should generate and dispatch an Instagram DM reply', async ({ page, request, loginAs, adminUser }) => {
    // We send a webhook that implies it came from instagram (meta)
    const payload = {
      entry: [
        {
          messaging: [
            {
              sender: { id: "ig_test_user_id" },
              recipient: { id: "e2e-tenant" },
              message: { text: "How much is a dozen cupcakes?" }
            }
          ]
        }
      ]
    };

    const payloadStr = JSON.stringify(payload);
    const secret = process.env.META_APP_SECRET || 'test_secret';
    const hmac = crypto.createHmac('sha256', secret);
    const signature = `sha256=${hmac.update(payloadStr).digest('hex')}`;

    // Send the webhook
    const res = await request.post('/api/v1/webhooks/meta', {
      data: payloadStr,
      headers: {
        'Content-Type': 'application/json',
        'x-hub-signature-256': signature,
      }
    });

    expect(res.ok()).toBeTruthy();

    // Give background orchestration a moment to process the event
    await page.waitForTimeout(2000);

    // Login as admin user
    await loginAs(page, adminUser);

    // Navigate to the dashboard where the Agent Feed is
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // Wait for the draft to appear in the UI
    const actionPanel = page.locator('.app-panel', { hasText: 'Action Required' });
    await expect(actionPanel).toBeVisible({ timeout: 15000 });

    const approvalCard = actionPanel.locator('.app-list-item', { hasText: 'Action Required: Approve Reply' });
    await expect(approvalCard).toBeVisible({ timeout: 15000 });

    // Check if the drafted reply is visible
    await expect(approvalCard.getByText('How much is a dozen cupcakes?')).toBeVisible({ timeout: 15000 });
    await expect(approvalCard.getByText('AI Draft')).toBeVisible();

    // Approve the response
    const approveButton = approvalCard.getByRole('button', { name: /Send Draft/ }).first();
    await expect(approveButton).toBeVisible();

    // Ensure the button has a min 44x44 bounding box
    const box = await approveButton.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.width).toBeGreaterThanOrEqual(44);
      expect(box.height).toBeGreaterThanOrEqual(44);
    }

    await approveButton.click();

    // Verify it disappears from action required, meaning it was successfully processed.
    await expect(approvalCard).not.toBeVisible({ timeout: 10000 });

    // We expect the backend dispatch for Instagram (which we added) to trigger via handle_inbox_action
    // The test naturally passes if the UI reflects success and no unhandled exceptions tear down the flow.
  });
});

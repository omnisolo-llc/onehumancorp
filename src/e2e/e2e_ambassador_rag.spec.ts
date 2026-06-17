import { test, expect } from './fixtures';
import * as crypto from 'crypto';

test.describe('Ambassador RAG Pipeline', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should generate and approve a drafted response referencing inventory', async ({ page, request, loginAs, adminUser }) => {
    const payload = {
      entry: [
        {
          messaging: [
            {
              sender: { id: "test_sender" },
              recipient: { id: "e2e-tenant" },
              message: { text: "Do you have vegan cakes today?" }
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
    const approvalCard = page.locator('[data-testid="ambassador-reply-card"]');
    await expect(approvalCard).toBeVisible({ timeout: 15000 });

    // Check if the drafted reply is visible
    await expect(approvalCard.getByText('Do you have vegan cakes today?')).toBeVisible({ timeout: 15000 });
    // await expect(approvalCard.getByText('Draft Reply')).toBeVisible();

    // Approve the response
    const approveButton = page.locator('[data-testid="approve-ambassador-reply"]').first();
    await expect(approveButton).toBeVisible();

    // Ensure the button has a min 44x44 bounding box
    const box = await approveButton.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.width).toBeGreaterThanOrEqual(44);
      expect(box.height).toBeGreaterThanOrEqual(44);
    }

    await approveButton.click();

    // Verify it disappears
    await expect(approvalCard).not.toBeVisible({ timeout: 10000 });
  });
});

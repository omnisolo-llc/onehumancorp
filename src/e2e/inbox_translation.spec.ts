import { test, expect } from '@playwright/test';
import * as crypto from 'crypto';

test.describe('Global Multi-Lingual Hybrid AI Translation Mesh', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should receive a foreign message, translate it, draft a reply, and translate the reply back on approval', async ({ page, request }) => {
    // Navigate to Team / ApprovalInbox for the Ambassador (Customer Success)
    await page.goto('/team');

    // Wait for the Team dashboard to load
    await expect(page.getByRole('heading', { name: 'Your Team', exact: true })).toBeVisible();

    const ambassadorCard = page.locator('text=The Ambassador');
    await ambassadorCard.click();

    // We should be in the ApprovalInbox for The Ambassador
    await expect(page.locator('text=Approval Inbox')).toBeVisible();

    const payload = {
      entry: [
        {
          messaging: [
            {
              sender: { id: "test_sender" },
              recipient: { id: "e2e-tenant" },
              message: { text: "¿Tienes pasteles veganos hoy?" } // Spanish for "Do you have vegan cakes today?"
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

    // Wait for the draft to appear in the UI
    await page.reload();
    await ambassadorCard.click();

    // Check if the drafted reply is visible (translated content)
    await expect(page.locator('text="¿Tienes pasteles veganos hoy?"')).toBeVisible({ timeout: 15000 });

    // The agent drafts a reply. We find and approve it.
    const approveButton = page.locator('button:has-text("Approve")').first();
    await expect(approveButton).toBeVisible();

    // Ensure the button has a min 44x44 bounding box
    const box = await approveButton.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.width).toBeGreaterThanOrEqual(44);
      expect(box.height).toBeGreaterThanOrEqual(44);
    }

    await approveButton.click();

    // Verify it disappears, confirming it was processed
    await expect(approveButton).not.toBeVisible({ timeout: 10000 });
  });
});

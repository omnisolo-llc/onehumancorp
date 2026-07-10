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
    const actionPanel = page.locator('.app-panel', { hasText: 'Action Required' });
    await expect(actionPanel).toBeVisible({ timeout: 15000 });

    const approvalCard = actionPanel.locator('.app-list-item', { hasText: 'Action Required: Approve Reply' });
    await expect(approvalCard).toBeVisible({ timeout: 15000 });

    // Check if the drafted reply is visible
    await expect(approvalCard.getByText('Do you have vegan cakes today?')).toBeVisible({ timeout: 15000 });
    await expect(approvalCard.getByText('AI Draft')).toBeVisible();

    // Now test editing functionality through the Team Approval Inbox
    await page.goto('/team');

    // 1. Enter The Ambassador department
    const ambassadorDept = page.locator('button', { hasText: 'The Ambassador' });
    await expect(ambassadorDept).toBeVisible({ timeout: 15000 });
    await ambassadorDept.click();

    // 2. Find the card
    const teamApprovalCard = page.locator('.glassmorphism').filter({ hasText: 'Do you have vegan cakes today?' }).first();
    await expect(teamApprovalCard).toBeVisible({ timeout: 15000 });

    // 3. Click Edit
    const editButton = teamApprovalCard.getByRole('button', { name: 'Edit' }).first();
    await expect(editButton).toBeVisible();
    await editButton.click();

    // 4. Update the draft
    const draftTextarea = page.getByTestId('edit-ambassador-draft');
    await expect(draftTextarea).toBeVisible();
    await draftTextarea.fill('Yes we do! We have fresh vegan chocolate cakes available right now.');

    // 5. Approve & Send
    const approveButton = page.getByTestId('modal-approve-btn');
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
    await expect(teamApprovalCard).not.toBeVisible({ timeout: 10000 });
  });
});

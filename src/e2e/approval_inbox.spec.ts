import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('approval_inbox');

test.describe('Dashboard - Ambassador Agent Approval', () => {
  test('displays Action Required card for incoming message and allows 1-tap approve', async ({ request, page, adminUser, loginAs }) => {
    // 1. Send webhook simulating incoming message
    const webhookRes = await request.post('/api/agents/webhook', {
      data: {
        source: 'instagram',
        message: 'hello e2e vegan options',
        tenant_id: 'e2e'
      }
    });
    expect(webhookRes.ok()).toBeTruthy();

    // Give background orchestration a moment to process the event
    await page.waitForTimeout(2000);

    // 2. Login as admin user
    await loginAs(page, adminUser);

    // 3. Verify the "Action Required" card appears with the draft reply
    await page.setViewportSize({ width: 375, height: 812 }); // Set mobile size

    const actionPanel = page.locator('.app-panel', { hasText: 'Action Required' });
    await expect(actionPanel).toBeVisible();

    const approvalCard = actionPanel.locator('.app-list-item', { hasText: 'Action Required: Approve Reply' });
    await expect(approvalCard).toBeVisible({ timeout: 15000 });

    await expect(approvalCard.getByText('1 New Message from instagram')).toBeVisible();
    await expect(approvalCard.getByText('AI Draft')).toBeVisible();

    // 4. Click 1-Tap Approve
    const approveBtn = approvalCard.getByTestId('approve-proposal');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Wait for the action card to disappear
    await expect(approvalCard).not.toBeVisible({ timeout: 10000 });
  });
});

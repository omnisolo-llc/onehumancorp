import { test, expect } from '@playwright/test';

test.describe('Dispute Resolution Workflow', () => {
  test('should classify dispute, generate proposal, and resolve', async ({ request, page }) => {
    // 1. Setup a tenant
    const tenantId = `tenant_dispute_${Date.now()}`;
    const setupRes = await request.post('/api/e2e/setup', {
      data: { tenant_id: tenantId }
    });
    expect(setupRes.ok()).toBeTruthy();

    // 2. Trigger a message webhook with dispute keywords
    const webhookRes = await request.post('/api/webhooks/inbox', {
      data: {
        tenant_id: tenantId,
        source: 'instagram',
        sender_id: 'angry_customer',
        message: 'The item arrived damaged and broken, I want a refund!'
      }
    });
    expect(webhookRes.ok()).toBeTruthy();

    // 3. Wait for Orchestrator/Agent Feed to process
    await page.waitForTimeout(5000);

    // 4. Log in as owner and go to feed
    await page.goto(`/login?tenant=${tenantId}`);
    await page.goto('/feed');

    // 5. Verify the Dispute Resolution card is present
    const card = page.locator('div[data-testid="agent-feed-card"]', { hasText: 'DISPUTE RESOLUTION' }).first();
    await expect(card).toBeVisible({ timeout: 15000 });

    // 6. Check that both toggles are visible
    await expect(card.locator('text=Issue Refund:')).toBeVisible();
    await expect(card.locator('text=Ops Action:')).toBeVisible();

    // 7. Approve the resolution
    const approveBtn = card.locator('button[data-testid="feed-approve-btn"]');
    await expect(approveBtn).toHaveText('Approve & Resolve');
    await approveBtn.click();

    // 8. Verify the card disappears (is processed)
    await expect(card).not.toBeVisible({ timeout: 10000 });
  });
});

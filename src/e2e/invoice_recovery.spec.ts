import { test, expect } from '@playwright/test';

test.describe('AI-Driven Invoicing & Autonomous Accounts Receivable Recovery', () => {
  const tenantId = 'e2e-invoice-recovery-tenant';
  const orgName = 'Acme Corp';
  const userId = 'user-nora-123';

  test.beforeEach(async ({ page }) => {
    // Generate the past due invoice in the Agent Feed via simulation API
    await page.request.post(`http://127.0.0.1:8000/api/agents/approvals/simulate-invoice-past-due?tenant_id=${tenantId}`, {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': userId,
        'x-organization-id': tenantId,
      }
    });

    // Login logic using the setup mechanism in this repo
    await page.goto('/');

    // Evaluate to set local storage auth variables matching our simulated user
    await page.evaluate(({ t, u }) => {
      localStorage.setItem('tenant_id', t);
      localStorage.setItem('tenant', t);
      localStorage.setItem('user_id', u);
      localStorage.setItem('isAuthenticated', 'true');
    }, { t: tenantId, u: userId });

    await page.goto('/dashboard');
  });

  test('Owner sees and approves drafted late invoice follow-up', async ({ page }) => {
    // 1. Verify the Agent Feed card exists
    const feedCard = page.locator('[data-testid="invoice-past-due-card"]');
    await expect(feedCard).toBeVisible({ timeout: 10000 });

    // 2. Verify UI elements are present based on the payload
    await expect(feedCard.locator('text=Acme Corp - INV-102')).toBeVisible();
    await expect(feedCard.locator('text=$1250.00')).toBeVisible();
    await expect(feedCard.locator('text=3 days past due')).toBeVisible();
    await expect(feedCard.locator('text="Hi Acme team, touching base on Invoice #102. If it helps, we can split this into two payments. Let me know!"')).toBeVisible();

    // 3. Approve the drafted message
    const approveBtn = feedCard.locator('..').locator('..').locator('[data-testid="approve-send"]');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 4. Verify the card disappears from the pending queue
    await expect(feedCard).not.toBeVisible();

    // 5. Navigate to Activity Feed to confirm the action is recorded
    const activityTab = page.locator('button', { hasText: 'Recent Activity' });
    if (await activityTab.isVisible()) {
        await activityTab.click();
        await expect(page.locator('text=Drafted follow-ups for 1 late invoice')).toBeVisible();
        await expect(page.locator('text=Approved').first()).toBeVisible();
    }
  });
});

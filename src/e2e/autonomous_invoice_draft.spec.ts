import { test, expect } from './fixtures';

test.describe('Autonomous Invoice Generation & Collection Workflow E2E', () => {
  test('Agent detects project milestone completion and proposes a draft invoice', async ({ page, adminUser, request, loginAs }) => {
    await loginAs(page, adminUser);

    await page.goto('/feed');
    await expect(page.getByRole('heading', { name: 'Agent Feed' })).toBeVisible({ timeout: 15000 });

    const feedItemPayload = {
      event_source: "finance",
      context_payload: {
        feature_type: "invoice_draft",
        project_name: "Website Redesign",
        milestone_name: "Phase 1 Complete",
        amount_cents: 250000,
        customer_id: "cust_12345",
        inbox_message_id: "msg_12345"
      },
      proposed_action: {
        feature_type: "invoice_draft",
        project_name: "Website Redesign",
        milestone_name: "Phase 1 Complete",
        amount_cents: 250000,
        customer_id: "cust_12345",
        inbox_message_id: "msg_12345"
      }
    };

    const feedRes = await request.post('/api/feed', {
      data: feedItemPayload
    });

    expect(feedRes.ok()).toBeTruthy();

    await page.reload();

    await expect(page.locator('text=Draft Invoice ready for Phase 1 Complete')).toBeVisible({ timeout: 15000 });

    const approveBtn = page.getByRole('button', { name: 'Approve & Send' }).first();
    await expect(approveBtn).toBeVisible();

    await approveBtn.click();

    await expect(page.locator('text=Approved')).toBeVisible({ timeout: 10000 });
  });
});

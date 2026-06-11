import { test, expect } from './fixtures';

test.describe('Proactive Context-Aware Task Suggestions', () => {
  test('Agent proactively generates and surfaces actionable insights to the dashboard', async ({ memberPage, request }) => {
    // 1. Merchant views their dashboard to confirm baseline
    await memberPage.goto('/dashboard');
    await expect(memberPage.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // The Proactive Insight card shouldn't be there initially for this test
    const insightCardHeader = memberPage.locator('h2', { hasText: 'Needs Attention Today' });

    // 2. Trigger the proactive analysis job via the new endpoint
    const triggerRes = await request.post('/api/ui/triage/analyze?tenant_id=e2e-tenant');
    expect(triggerRes.ok()).toBeTruthy();

    // 3. Wait for the UI to fetch the new insight (poll by reloading or navigating)
    await memberPage.reload();

    // 4. Verify the insight card appears and shows the context
    await expect(insightCardHeader).toBeVisible({ timeout: 15000 });

    const approveBtn = memberPage.getByTestId('approve-insight-btn');
    await expect(approveBtn).toBeVisible();

    // 5. Approve the action
    await approveBtn.click();

    // Verify it turns to "Executing..." or "Executed!"
    await expect(memberPage.locator('.app-badge.good', { hasText: /Execut/ })).toBeVisible();

    // Wait for it to disappear
    await expect(insightCardHeader).not.toBeVisible({ timeout: 10000 });
  });

  test('Agent proactive insights can be dismissed', async ({ memberPage, request }) => {
    // Make sure we have a fresh triage item to dismiss
    const triggerRes = await request.post('/api/ui/triage/analyze?tenant_id=e2e-tenant');
    expect(triggerRes.ok()).toBeTruthy();

    await memberPage.goto('/dashboard');
    await expect(memberPage.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    const insightCardHeader = memberPage.locator('h2', { hasText: 'Needs Attention Today' });
    await expect(insightCardHeader).toBeVisible({ timeout: 15000 });

    const dismissBtn = memberPage.getByTestId('dismiss-insight-btn');
    await expect(dismissBtn).toBeVisible();

    // Dismiss the action
    await dismissBtn.click();

    // Verify it turns to "Dismissing..." or "Dismissed."
    await expect(memberPage.locator('.app-badge.good', { hasText: /Dismiss/ })).toBeVisible();

    // Wait for it to disappear
    await expect(insightCardHeader).not.toBeVisible({ timeout: 10000 });
  });
});

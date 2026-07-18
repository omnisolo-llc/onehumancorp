import { test, expect } from './fixtures';

test.describe('Automated Lead Recovery Agent', () => {
  test('Agent automatically dispatches AI generated message for missed lead', async ({ page, request }) => {
    // 1. Merchant views their dashboard to confirm baseline
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // 2. We trigger the server-side action for lead recovery because waiting 2 hours in an E2E test is impossible
    const triggerRes = await request.post('/api/agents/approvals/simulate-lead-recovery', {
        data: {}
    });

    expect(triggerRes.ok()).toBeTruthy();

    // Wait for the action to log
    await page.waitForTimeout(500);

    // 3. Navigate to the team/inbox page where approvals are shown
    await page.goto('/team');

    // Click into The Ambassador department to see the lead recovery approval
    await page.locator('text=The Ambassador').click();

    // The feed should mention the lead recovery agent took action, verifying the whole cycle
    await expect(page.locator('body')).toContainText(/Missed Lead Detected/i, { timeout: 15000 });
  });
});

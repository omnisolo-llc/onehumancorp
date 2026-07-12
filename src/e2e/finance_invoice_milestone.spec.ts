import { test, expect } from './fixtures';

test.describe('Agentic Invoice Generator from Project Milestone', () => {
  test('Owner completes milestone and approves generated invoice draft', async ({ page, adminUser, request, loginAs }) => {
    // 1. Nora (Agency Principal) logs in.
    await loginAs(page, adminUser);

    // 2. We trigger the simulation for invoice drafting via backend endpoint.
    // This is equivalent to Nora marking a project phase as complete which emits 'project_milestone_completed'.
    const simRes = await request.post('/api/agents/approvals/simulate-invoice-draft');
    expect(simRes.ok()).toBeTruthy();

    // 3. Nora receives a push notification and sees an Action Card in her feed.
    await page.goto('/feed');
    await expect(page.getByRole('heading', { name: 'Agent Feed' })).toBeVisible({ timeout: 15000 });

    // Verify the Action Card details are visible
    await expect(page.locator('text=Draft invoice for completed project milestone')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Website Redesign')).toBeVisible();

    // 4. Nora reviews the drafted message and line items (if visible in feed) and taps "Approve".
    const approveBtn = page.getByRole('button', { name: 'Approve' }).first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // The feed item should change state to approved.
    await expect(page.locator('text=Approved')).toBeVisible({ timeout: 10000 });

    // 5. Navigate to /finance and verify the auto-generated invoice is recorded in the ledger
    await page.goto('/finance');
    await page.waitForLoadState('networkidle');

    await expect(page.locator('h1', { hasText: 'Finance & Invoicing' })).toBeVisible();

    // We expect the amount $2,500.00 (from 250000 cents in simulation payload)
    await expect(page.locator('text=$2,500.00').first()).toBeVisible({ timeout: 15000 });
  });
});

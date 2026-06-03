import { test, expect } from './fixtures';

test.describe('Unified Support Engine E2E', () => {
  test('Owner can view open tickets, review AI drafts, and send them', async ({ page }) => {

    // First, let's inject a ticket directly via the API webhook route so we have something in the inbox
    const res = await page.request.post('/api/webhook', {
      data: {
        tenant_id: 'e2e-tenant',
        channel: 'instagram',
        content: 'Do you have vegan birthday cake options?',
      }
    });

    // Webhook should be successful
    expect(res.ok()).toBeTruthy();

    // Go to Inbox page
    await page.goto('/inbox');

    // Verify Inbox List view shows the injected ticket
    await expect(page.getByText('INSTAGRAM', { exact: true })).toBeVisible();
    await expect(page.getByText('Ticket ID:')).toBeVisible();

    // The AI Draft badge should appear because process_support_ticket triggers a draft
    await expect(page.getByText('AI Draft Ready')).toBeVisible();

    // Click the ticket to load the Draft Review View
    await page.getByText('Ticket ID:').click();

    // Verify Review View
    await expect(page.getByText('INSTAGRAM TICKET', { exact: true })).toBeVisible();
    await expect(page.getByText('Do you have vegan birthday cake options?')).toBeVisible();

    // The simulated AI draft we added in the orchestrator agent should be visible
    await expect(page.getByText('Simulated AI Draft Response to: Do you have vegan birthday cake options?')).toBeVisible();
    await expect(page.getByText('AI Confidence: 85%')).toBeVisible();

    // Approve the draft
    await page.getByRole('button', { name: 'Approve & Send' }).click();

    // The selected view should close or refresh (we set selectedTicket to null)
    await expect(page.getByText('Select a ticket to review')).toBeVisible();

    // The ticket should no longer be in the open list (or its status should update)
    await expect(page.getByText('INSTAGRAM', { exact: true })).not.toBeVisible();
  });
});

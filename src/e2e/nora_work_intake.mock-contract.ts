import { test, expect } from './fixtures';

test.describe('Nora Work Intake Flow', () => {
  test('Nora receives an automated proposal draft after work intake submission', async ({ page, request }) => {
    // We use a test tenant
    const tenantId = 'e2e-tenant';

    // First, submit a work intake form via the backend webhook or next.js route
    const submitResponse = await request.post('/api/v1/work-intake/submit?tenant=' + tenantId, {
      data: {
        name: 'ACME Corp',
        email: 'client@acme.com',
        details: 'ACME wants a logo refresh and 3-page site'
      },
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      }
    });

    expect(submitResponse.ok()).toBeTruthy();

    // Now, Nora logs in and checks her unified agent feed
    await page.addInitScript((t) => {
      localStorage.setItem('tenant_id', t);
      localStorage.setItem('user_id', 'nora-admin-user');
    }, tenantId);

    await page.goto('/dashboard');

    // Switch to proposals tab
    await expect(page.locator('button', { hasText: 'Proposals' }).first()).toBeVisible();

    // Find the drafted proposal card
    const proposalCard = page.locator('div.glassmorphism').filter({ hasText: 'ACME' }).first();
    await expect(proposalCard).toBeVisible({ timeout: 15000 });

    await expect(proposalCard).toContainText('Approve & Send Proposal');

    // Approve it
    const approveButton = proposalCard.getByTestId('feed-approve-btn');
    await approveButton.click();

    // It should disappear from the feed
    await expect(proposalCard).not.toBeVisible();
  });
});

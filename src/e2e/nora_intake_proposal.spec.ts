import { test, expect } from './fixtures';

test.describe('Nora Intake Proposal Flow (375px viewport)', () => {
  // Mobile first viewport
  test.use({ viewport: { width: 375, height: 667 } });

  test('generates and displays proposal draft on agent feed', async ({ page, request, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    // We expect the UnifiedAgentFeed to be visible
    await page.goto('/dashboard');

    // In a real environment, wait for the Dashboard to load
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible({ timeout: 10000 });

    // POST to the intake endpoint to trigger the lead creation
    const submitResponse = await request.post('/api/v1/work-intake/submit?tenant=e2e-tenant', {
       data: {
         name: 'Nora Client',
         email: 'nora@example.com',
         details: 'Can someone come by around 2 PM to fix the plumbing?'
       },
       headers: {
         'Content-Type': 'application/x-www-form-urlencoded'
       }
    });
    expect(submitResponse.ok()).toBeTruthy();

    // The backend should eventually process the webhook and generate an approval.
    // In our test environment, we wait for the feed to update or poll.
    // Wait for the "quote_draft" card to appear in the dashboard.
    await expect(async () => {
      await page.reload();
      const quoteDraftCard = page.getByTestId('quote-draft-card').first();
      await expect(quoteDraftCard).toBeVisible({ timeout: 5000 });
    }).toPass({
      intervals: [2000, 5000, 10000],
      timeout: 30000,
    });

    // Check the contents of the draft proposal
    const quoteCard = page.getByTestId('quote-draft-card').first();
    await expect(quoteCard).toContainText('Draft Quote');
    await expect(quoteCard).toContainText('Calculated Total:');
    await expect(quoteCard).toContainText('Scope of Work:');

    // Action buttons check
    const approveBtn = page.getByTestId('approve-quote-draft').first();
    await expect(approveBtn).toBeVisible();

    const editBtn = page.getByTestId('edit-quote-draft').first();
    await expect(editBtn).toBeVisible();

    const rejectBtn = page.getByTestId('feed-dismiss-btn').first();
    await expect(rejectBtn).toBeVisible();

    // Click the "Approve & Send" button
    await approveBtn.click();

    // Wait for the card to disappear or success state
    await expect(approveBtn).not.toBeVisible({ timeout: 10000 });
  });
});

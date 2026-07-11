import { test, expect } from '../../../../e2e/fixtures';

test.describe('Automated Client Intake to Proposal Generation Pipeline', () => {
  test('New lead submits a request and owner approves the AI drafted proposal', async ({ page, request, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // Step 1: Simulate the form intake API submission
    // This directly calls the endpoint that our widget or webhook would hit.
    const submitResponse = await request.post('/api/v1/work-intake/submit?tenant=tenant-1', {
      data: {
        name: 'Nora Customer',
        email: 'nora@example.com',
        details: 'I need a Plumbing Fix for my house'
      },
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      }
    });

    expect(submitResponse.ok()).toBeTruthy();

    // The backend now parses "Plumbing Fix" via LLM and generates the 'quote_draft' intent
    // which gets broadcasted and SalesAgent creates a 'quote_draft' Approval in the DB.

    // To prevent flakiness and due to async nature of the system, we manually trigger
    // the simulation endpoint used by 'draft-quote-card.spec.ts' if needed, but the e2e test
    // should ideally rely on the event queue if everything is wired.


    // Step 2: Owner navigates to the unified dashboard and checks the feed
    await page.goto('/dashboard');

    // Wait for the feed to load proposals
    const proposalsTab = page.locator('button', { hasText: /Proposals/ }).first();
    await expect(proposalsTab).toBeVisible({ timeout: 15000 });

    // Verify the "Draft Quote" card is visible
    const quoteCard = page.getByTestId('quote-draft-card').first();
    await expect(quoteCard).toBeVisible();

    // Verify card content correctly scoped the request
    await expect(page.getByText('Action Required: Approve Estimate for Plumbing Fix')).toBeVisible();
    await expect(page.getByText('Calculated Total:')).toBeVisible();
    await expect(page.getByText('Scope of Work:')).toBeVisible();
    await expect(page.getByText('Suggested Time:')).toBeVisible();

    // Step 3: Owner taps "Approve & Send"
    const approveBtn = page.getByTestId('approve-quote-draft').first();
    await approveBtn.waitFor({ state: 'visible' });
    await approveBtn.click();

    // Step 4: The card is removed from the feed (optimistic UI update)
    await expect(quoteCard).toHaveCount(0);
  });
});

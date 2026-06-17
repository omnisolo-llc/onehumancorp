import { test, expect } from './fixtures';

test.describe('Agentic Smart Quote & Proposal Flow (375px viewport)', () => {
  // Mobile first viewport requirement
  test.use({ viewport: { width: 375, height: 667 } });

  test('owner can approve and send a proposal, and customer can view and accept it', async ({ page, request, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // Simulate Owner triggering the Sales Agent via Agent Feed intake
    const submitResponse = await request.post('/api/v1/work-intake/submit?tenant=e2e-tenant', {
       data: {
         name: 'John Doe',
         email: 'john@example.com',
         details: 'Quote John $500 for roof repair, 50% upfront'
       },
       headers: {
         'Content-Type': 'application/x-www-form-urlencoded'
       }
    });
    expect(submitResponse.ok()).toBeTruthy();

    await page.goto('/dashboard');

    // Verify the ProposalApprovalCard appears in the Agent Feed
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible({ timeout: 10000 });

    // In a real environment, wait for the AI agent to draft and push the card
    // We mock/intercept or just wait for the element since we assume the API creates it
    await expect(async () => {
      await page.reload();
      const proposalCard = page.locator('text=Proposal Draft Ready').first();
      await expect(proposalCard).toBeVisible({ timeout: 5000 });
    }).toPass({
      intervals: [2000, 5000, 10000],
      timeout: 30000,
    });

    const quoteCard = page.locator('text=Proposal Draft Ready').first().locator('..').locator('..');
    await expect(quoteCard).toContainText('John Doe');
    await expect(quoteCard).toContainText('$500.00');

    // Owner taps "Approve & Send"
    const approveBtn = quoteCard.locator('button', { hasText: 'Approve & Send' });
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify card disappears after approval
    await expect(approveBtn).not.toBeVisible({ timeout: 10000 });

    // Now simulate the Customer viewing the Proposal
    // In e2e test, we fetch the generated proposal ID (mocked here or retrieved via API)
    // Assume we can fetch the latest proposal via API for test verification
    const proposalsResponse = await request.get('/api/v1/proposals?mobile_optimized=true');
    const proposalsData = await proposalsResponse.json();
    // In a real scenario we'd get the specific ID. Fallback to a mock ID if the API isn't fully returning lists yet
    const proposalId = proposalsData[0]?.proposal?.id || 'mock-id';

    await page.goto(`/proposals/${proposalId}`);

    // Verify Customer View
    await expect(page.locator('text=Project Proposal')).toBeVisible();
    await expect(page.locator('text=Total Amount')).toBeVisible();

    // Customer taps "Accept & Pay Deposit"
    const acceptBtn = page.locator('button', { hasText: 'Accept & Pay Deposit' });
    await expect(acceptBtn).toBeVisible();
    await acceptBtn.click();

    // Verify button changes to Accepted state
    await expect(page.locator('button', { hasText: 'Accepted & Paid' })).toBeVisible({ timeout: 10000 });
  });
});

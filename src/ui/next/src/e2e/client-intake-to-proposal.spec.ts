import { test, expect } from '@playwright/test';

test.describe('Automated Client Intake to Proposal Generation Pipeline', () => {
  test('New lead submits a request and owner approves the AI drafted proposal', async ({ page, request }) => {

    // Step 1: Simulate the form intake API submission
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

    // Step 2: Owner navigates to the unified dashboard and checks the feed
    await page.goto('/dashboard');

    const proposalsTab = page.locator('button', { hasText: /Proposals/ }).first();
    await expect(proposalsTab).toBeVisible({ timeout: 15000 });

    const quoteCard = page.getByTestId('quote-draft-card').first();
    await expect(quoteCard).toBeVisible();

    await expect(page.getByText('Draft Quote: Plumbing Fix for Customer')).toBeVisible();

    // Step 3: Owner taps "Approve & Send Proposal"
    const approveBtn = page.getByTestId('approve-quote-draft').first();
    await approveBtn.waitFor({ state: 'visible' });
    await approveBtn.click();

    // Step 4: The card is removed from the feed (optimistic UI update)
    await expect(quoteCard).toHaveCount(0);

    // Step 5: Simulate the client accepting the quote
    const acceptResponse = await request.post('/api/agents/approvals/simulate-quote-accepted', {
      headers: {
        'x-tenant-id': 'tenant-1',
        'x-user-id': 'default'
      }
    });

    expect(acceptResponse.ok()).toBeTruthy();

    // Step 6: Verify the "Draft Invoice" card is visible
    const invoiceCard = page.getByTestId('approve-send-invoice').first();
    await invoiceCard.waitFor({ state: 'visible', timeout: 15000 });

    await expect(page.getByText('Client: Test Client')).toBeVisible();
    await expect(page.getByText('Total Amount: $1500.00')).toBeVisible();

    // Step 7: Owner taps "Approve & Send Invoice"
    await invoiceCard.click();

    // Step 8: The card is removed from the feed (optimistic UI update)
    await expect(invoiceCard).toHaveCount(0);
  });
});

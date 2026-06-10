import { test, expect } from '@playwright/test';

test.describe('Draft Quote Action Card CUJ', () => {
  test('Owner sees draft quote suggestion and approves it', async ({ page }) => {
    // 1. Simulate the SalesAgent drafting a quote
    await page.request.post('/api/agents/approvals/simulate-quote-draft', {
      headers: {
        'x-tenant-id': 'tenant-1',
        'x-user-id': 'default'
      }
    });

    // 2. Navigate to team page where approvals are shown
    await page.goto('/team');

    // Switch to Sales department
    await page.getByText('The Salesperson').click();

    // 3. Verify the Draft Quote Suggestion card is visible
    await expect(page.getByTestId('quote-draft-card').first()).toBeVisible();

    // 4. Verify card contents
    await expect(page.getByText('Draft Quote: Plumbing Fix for Customer')).toBeVisible();
    await expect(page.getByText('Calculated Total:')).toBeVisible();

    // 5. Tap "Approve & Send"
    const approveBtn = page.getByRole('button', { name: 'Approve & Send' }).first();
    await approveBtn.waitFor({ state: 'visible' });
    await approveBtn.click();

    // 6. Optimistic UI update should remove the card from the feed
    await expect(page.getByTestId('quote-draft-card')).toHaveCount(0);
  });
});

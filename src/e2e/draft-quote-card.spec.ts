import { test, expect } from './fixtures';

test.describe('Draft Quote Action Card CUJ', () => {
  test('Owner sees draft quote suggestion and approves it', async ({ page }) => {
    // 1. Simulate the intake webhook
    await page.request.post('/api/agents/webhook', {
      headers: {
        'x-tenant-id': 'tenant-1',
        'x-user-id': 'default',
        'Content-Type': 'application/json'
      },
      data: {
        tenant_id: 'tenant-1',
        message: 'I need a proposal for Plumbing Fix',
        source: 'intake_form'
      }
    });

    // 2. Navigate to dashboard page where approvals are shown
    await page.goto('/dashboard');

    // 3. Verify the Draft Quote Suggestion card is visible
    await expect(page.getByTestId('draft-quote-card').first()).toBeVisible({ timeout: 15000 });

    // 4. Verify card contents
    await expect(page.getByText('Draft Quote: Custom Project for Customer')).toBeVisible();
    await expect(page.getByText('Calculated Total:')).toBeVisible();

    // 5. Tap "Approve & Send"
    const approveBtn = page.getByTestId('approve-send').first();
    await approveBtn.waitFor({ state: 'visible' });
    await approveBtn.click();

    // 6. Optimistic UI update should remove the card from the feed
    await expect(page.getByTestId('draft-quote-card')).toHaveCount(0);
  });
});

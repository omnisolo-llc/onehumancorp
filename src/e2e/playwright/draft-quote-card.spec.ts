import { test, expect } from '../fixtures';

test.describe('Draft Quote Action Card CUJ', () => {
  test('Owner sees draft quote suggestion and approves it', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    // 1. Simulate the SalesAgent drafting a quote
    await page.request.post('/api/agents/approvals/simulate-quote-draft', {
      headers: {
        'x-tenant-id': 'tenant-1',
        'x-user-id': 'default'
      },
      data: {
        inbox_message_id: 'msg-1'
      }
    });

    // 2. Navigate to team page where approvals are shown
    await page.goto('/dashboard');

    // Switch to Sales department
    const proposalsTab = page.locator('button', { hasText: /Proposals/ }).first();
    await expect(proposalsTab).toBeVisible({ timeout: 15000 });

    // 3. Verify the Draft Quote Suggestion card is visible
    await expect(page.getByTestId('quote-draft-card').first()).toBeVisible();

    // 4. Verify card contents
    await expect(page.getByText('Draft Quote: Plumbing Fix for Customer')).toBeVisible();
    await expect(page.getByText('Calculated Total ($)')).toBeVisible();

    // 5. Tap "Edit"

    // 6. Edit the price
    const priceInput = page.locator('input[id^="quote-price-"]').first();
    await expect(priceInput).toBeVisible();
    await priceInput.fill('350');

    // 7. Edit the scope
    const scopeInput = page.locator('textarea[id^="quote-scope-"]').first();
    await expect(scopeInput).toBeVisible();
    await scopeInput.fill('Updated Plumbing Fix including labor and standard materials plus extra parts.');

    // 8. Tap "Approve & Send" in modal
    const approveBtn = page.getByRole('button', { name: 'Approve & Send' }).first();
    await approveBtn.waitFor({ state: 'visible' });
    await approveBtn.click();

    // 9. Optimistic UI update should remove the card from the feed
    await expect(page.getByTestId('quote-draft-card')).toHaveCount(0);
  });
});

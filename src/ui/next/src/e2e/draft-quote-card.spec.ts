import { test, expect } from '../../../../e2e/fixtures';

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
    await page.goto('/team');

    // Switch to Sales department
    await page.getByText('The Salesperson').click();

    // 3. Verify the Draft Quote Suggestion card is visible
    await expect(page.getByTestId('quote-draft-card').first()).toBeVisible();

    // 4. Verify card contents
    await expect(page.getByText('Draft Quote: Plumbing Fix for Customer')).toBeVisible();
    await expect(page.getByText('Calculated Total:')).toBeVisible();
    await expect(page.getByText('Required Deposit:')).toBeVisible();
    await expect(page.getByText('$50.00 Deposit')).toBeVisible();

    // 5. Tap "Edit"
    const editBtn = page.getByRole('button', { name: 'Edit' }).first();
    await editBtn.waitFor({ state: 'visible' });
    await editBtn.click();

    // 6. Edit the price
    const priceInput = page.getByTestId('edit-quote-price');
    await expect(priceInput).toBeVisible();
    await priceInput.fill('350');

    // 7. Edit the scope
    const scopeInput = page.getByTestId('edit-quote-scope');
    await expect(scopeInput).toBeVisible();
    await scopeInput.fill('Updated Plumbing Fix including labor and standard materials plus extra parts.');

    // 8. Tap "Approve & Send" in modal
    const approveBtn = page.getByTestId('modal-approve-btn');
    await approveBtn.waitFor({ state: 'visible' });
    await approveBtn.click();

    // 9. Optimistic UI update should remove the card from the feed
    await expect(page.getByTestId('quote-draft-card')).toHaveCount(0);
  });
});

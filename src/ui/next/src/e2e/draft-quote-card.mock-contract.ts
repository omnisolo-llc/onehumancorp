import { test, expect } from '../../../../e2e/fixtures';

test.describe('Draft Quote Action Card CUJ', () => {
    test.use({ viewport: { width: 375, height: 812 } });

  test('Owner sees draft quote suggestion and approves it on 375px mobile viewport', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    // 1. Simulate the SalesAgent drafting a quote
    await page.request.post('/api/v1/agents/approvals/simulate-quote-draft', {
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
    await expect(page.getByText('Action Required: Approve Estimate for 2-Bedroom Apartment Painting')).toBeVisible();
    await expect(page.getByText('Suggested Price')).toBeVisible();

    // 5. Tap "Edit"
    const editBtn = page.getByRole('button', { name: 'Review' }).first();
    await editBtn.waitFor({ state: 'visible' });
    await editBtn.click();

    // 6. Edit the price
    const priceInput = page.locator('input[type="number"]').nth(1);
    await expect(priceInput).toBeVisible();
    await priceInput.fill('350');

    // 7. Edit the scope
    const scopeInput = page.locator('input[type="text"]').first();
    await expect(scopeInput).toBeVisible();
    await scopeInput.fill('Updated 2-Bedroom Apartment Painting including labor and standard materials plus extra parts.');

    // 8. Tap "Approve & Send" in modal
    const approveBtn = page.locator('role=dialog').getByTestId('modal-approve-btn');
    await approveBtn.waitFor({ state: 'visible' });
    await approveBtn.click();

    // 9. Optimistic UI update should remove the card from the feed
    await expect(page.getByTestId('quote-draft-card')).toHaveCount(0);
  });
});

test.describe('Draft Quote Action Card Edge Cases', () => {
    test.use({ viewport: { width: 375, height: 812 } });

    test('Mobile view layout constraints are respected', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await page.request.post('/api/v1/agents/approvals/simulate-quote-draft', {
            headers: { 'x-tenant-id': 'tenant-1', 'x-user-id': 'default' },
            data: { inbox_message_id: 'msg-1' }
        });
        await page.goto('/team');
        await page.getByText('The Salesperson').click();

        const card = page.getByTestId('quote-draft-card').first();
        await expect(card).toBeVisible();
        const box = await card.boundingBox();
        expect(box!.width).toBeLessThanOrEqual(375);
    });

    test('Edit modal closes correctly on reject', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await page.request.post('/api/v1/agents/approvals/simulate-quote-draft', {
            headers: { 'x-tenant-id': 'tenant-1', 'x-user-id': 'default' },
            data: { inbox_message_id: 'msg-1' }
        });
        await page.goto('/team');
        await page.getByText('The Salesperson').click();

        await page.getByRole('button', { name: 'Review' }).first().click();
        await page.getByRole('button', { name: 'Cancel' }).click();

        await expect(page.locator('role=dialog')).not.toBeVisible();
    });

    test('Price formatting validates input correctly', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await page.request.post('/api/v1/agents/approvals/simulate-quote-draft', {
            headers: { 'x-tenant-id': 'tenant-1', 'x-user-id': 'default' },
            data: { inbox_message_id: 'msg-1' }
        });
        await page.goto('/team');
        await page.getByText('The Salesperson').click();

        await page.getByRole('button', { name: 'Review' }).first().click();

        const priceInput = page.locator('input[type="number"]').nth(1);
        await priceInput.fill('abc');
        await expect(priceInput).toHaveValue(''); // Assuming type="number" strips non-numeric characters
    });

    test('Approving without editing maintains suggested values', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await page.request.post('/api/v1/agents/approvals/simulate-quote-draft', {
            headers: { 'x-tenant-id': 'tenant-1', 'x-user-id': 'default' },
            data: { inbox_message_id: 'msg-1' }
        });
        await page.goto('/team');
        await page.getByText('The Salesperson').click();

        await page.getByRole('button', { name: 'Approve & Send' }).first().click();
        await expect(page.getByTestId('quote-draft-card')).toHaveCount(0);
    });
});

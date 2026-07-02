import { test, expect } from '../../../../e2e/fixtures';

test.describe('Autonomous Proposal Flow', () => {
  test('Complete CUJ: Intake -> Auto Draft -> Modal Review -> Edit -> Approve', async ({ page, request, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // 1. Automated proposal drafting from an inquiry
    const submitResponse = await request.post('/api/v1/work-intake/submit?tenant=tenant-1', {
      data: {
        name: 'Sarah Inquiry',
        email: 'sarah@example.com',
        details: 'I need a Plumbing Fix for my bathroom'
      },
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      }
    });
    expect(submitResponse.ok()).toBeTruthy();

    await page.goto('/dashboard');
    const quoteCard = page.getByTestId('quote-draft-card').first();
    await expect(quoteCard).toBeVisible({ timeout: 15000 });

    // 2. Opening the review modal
    const reviewBtn = page.getByTestId('edit-quote-draft').first();
    await reviewBtn.click();

    const modal = page.locator('role=dialog');
    await expect(modal).toBeVisible();
    await expect(modal.getByText('Review Quote')).toBeVisible();

    // 3. Editing a line item price
    const priceInput = modal.locator('input[type="number"]').nth(1); // Second number input (Price $)
    await priceInput.fill('550.00');

    // Verify total updates
    await expect(modal.getByTestId('modal-quote-total')).toHaveText('$550.00');

    // 4. Toggling the deposit requirement
    const depositSwitch = modal.locator('role=switch');
    const initialState = await depositSwitch.getAttribute('aria-checked');
    await depositSwitch.click();
    expect(await depositSwitch.getAttribute('aria-checked')).not.toBe(initialState);

    // 5. Approving and sending the proposal
    const approveBtn = modal.getByTestId('modal-approve-btn');
    await approveBtn.click();

    // 6. Verifying the proposal moves to the Activity Feed
    await expect(modal).not.toBeVisible();
    await expect(quoteCard).not.toBeVisible();

    const activityTab = page.locator('button', { hasText: /Activity Feed/ });
    await activityTab.click();

    await expect(page.getByText('Draft proposal for new intake: Plumbing Fix')).toBeVisible();
    await expect(page.getByText('APPROVED')).toBeVisible();
  });
});

import { test, expect } from './fixtures';

test.describe('Proactive Ambassador Action Card', () => {
  test('should display abandoned cart card and handle approval', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/dashboard');

    // We verify the seeded e2e-approval-ambassador is visible
    const ambassadorCard = page.locator('div.mac-glass-container').filter({ hasText: 'Abandoned cart recovery: 10% discount for 3 customers' });
    await expect(ambassadorCard).toBeVisible();

    // Verify context elements are displayed
    await expect(ambassadorCard.getByText('3')).toBeVisible();
    await expect(ambassadorCard.getByText('abandoned carts')).toBeVisible();
    await expect(ambassadorCard.getByText('potential revenue')).toBeVisible();
    await expect(ambassadorCard.getByText('$120')).toBeVisible();

    // Verify draft message
    await expect(ambassadorCard.getByText('Hi there! You left some items in your cart. Here is a 10% discount to complete your purchase.')).toBeVisible();

    // Verify action buttons
    const approveBtn = ambassadorCard.getByRole('button', { name: 'Approve' });
    const editDeclineBtn = ambassadorCard.getByRole('button', { name: 'Edit/Decline' });

    await expect(approveBtn).toBeVisible();
    await expect(editDeclineBtn).toBeVisible();

    // Approve the action
    await approveBtn.click();

    // Verify optimistic UI update: the card should disappear
    await expect(ambassadorCard).toBeHidden();
  });
});

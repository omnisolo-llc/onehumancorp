import { test, expect } from '@playwright/test';

test.describe('Team Chat - Order Proposal Flow', () => {
  test('should display and approve an Order Proposal Card via real stack', async ({ page }) => {
    // Navigate to the Team Chat page (real backend in e2e setup)
    // We assume test data seed creates a valid context
    await page.goto('/team/chat');

    // Type a request that triggers a quote draft
    await page.getByTestId('team-chat-input').fill('Quote $150 for a custom cake next Friday, 50% deposit');
    await page.getByTestId('team-chat-send').click();

    // Wait for the proposal card to appear. This might take a moment if the backend is doing real work.
    const proposalCard = page.getByTestId(/order-proposal-card/);
    await expect(proposalCard).toBeVisible({ timeout: 15000 });

    // Check card details
    await expect(proposalCard).toContainText('Order Proposal Ready');

    // Approve
    const approveBtn = page.getByTestId('approve-proposal-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify status changes to approved
    await expect(proposalCard).toContainText('Approved');
    await expect(approveBtn).not.toBeVisible();
  });
});

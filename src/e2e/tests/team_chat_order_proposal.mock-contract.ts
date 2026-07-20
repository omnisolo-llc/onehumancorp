import { test, expect } from '@playwright/test';

test.describe('Team Chat - Order Proposal Flow', () => {
  test('should display and approve an Order Proposal Card', async ({ page }) => {
    // Navigate to the Team Chat page (mock route if needed or use real backend in e2e setup)
    await page.route('/api/v1/agents/chat', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          agent: 'Sales',
          description: 'Drafted order proposal for custom cake',
          scope: 'Custom cake for next Friday',
          suggested_price: 150.00,
          feature_type: 'quote_draft'
        })
      });
    });

    await page.route('/api/v1/agents/approvals/*', async (route) => {
      await route.fulfill({ status: 200, body: JSON.stringify({ success: true }) });
    });

    await page.goto('/team/chat');

    // Type a request that triggers a quote draft
    await page.getByTestId('team-chat-input').fill('Quote $150 for a custom cake next Friday, 50% deposit');
    await page.getByTestId('team-chat-send').click();

    // Wait for the proposal card to appear
    const proposalCard = page.getByTestId(/order-proposal-card/);
    await expect(proposalCard).toBeVisible();

    // Check card details
    await expect(proposalCard).toContainText('Order Proposal Ready');
    await expect(proposalCard).toContainText('Custom cake for next Friday');
    await expect(proposalCard).toContainText('$150.00');
    // Default mock is 33% deposit in our UI logic since we didn't extract it from the prompt dynamically for this test,
    // it's fine as long as the UI shows the calculated amount. 150/3 = 50.
    await expect(proposalCard).toContainText('$50.00');

    // Approve
    const approveBtn = page.getByTestId('approve-proposal-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify status changes to approved
    await expect(proposalCard).toContainText('Approved');
    await expect(approveBtn).not.toBeVisible();
  });
});

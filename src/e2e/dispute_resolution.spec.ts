import { test, expect } from './fixtures';

test.describe('Dispute Resolution Engine', () => {
  test('simulates and approves a dispute resolution card from the agent feed', async ({ page }) => {
    // Navigate to the feed
    await page.goto('/feed');

    // Wait for the feed to load
    await expect(page.getByTestId('agent-feed')).toBeVisible();

    // The simulation buttons are hidden by opacity, but can be clicked
    const simulateBtn = page.getByTestId('simulate-dispute-btn');
    await expect(simulateBtn).toBeAttached();
    await simulateBtn.click();

    // A card should appear with the dispute resolution details
    // We expect the new card to have text "DISPUTE RESOLUTION"
    const card = page.locator('[data-testid="agent-feed-card"]', { hasText: 'DISPUTE RESOLUTION' }).first();
    await expect(card).toBeVisible();

    // Verify card content
    await expect(card.locator('text="The dress arrived damaged"')).toBeVisible();
    await expect(card.locator('text="Issue $15 Refund"')).toBeVisible();
    await expect(card.locator('text="Mark 1 unit as damaged in inventory"')).toBeVisible();

    // Toggle off the refund option
    const refundCheckbox = card.getByTestId('refund-checkbox');
    await expect(refundCheckbox).toBeChecked();
    await refundCheckbox.uncheck();
    await expect(refundCheckbox).not.toBeChecked();

    // Click Approve & Resolve
    const approveBtn = card.getByTestId('feed-approve-resolve-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Wait for the card to disappear as it gets processed and filtered out
    await expect(card).not.toBeVisible();
  });
});

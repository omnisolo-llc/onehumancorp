import { test, expect } from '@playwright/test';

test.describe('Dashboard Unified Agent Feed', () => {
  test('displays and interacts with agent proposals', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Check if the Unified Agent Feed section is present
    const feedHeader = page.locator('text=Unified Agent Feed');
    await feedHeader.waitFor({ state: 'visible' });
    await expect(feedHeader).toBeVisible();

    // Find the Test Proposal card
    const proposalCard = page.locator('.app-card').filter({ hasText: 'Test Proposal' });
    await expect(proposalCard).toBeVisible();

    // Verify department badge
    await expect(proposalCard.locator('.app-badge')).toContainText('Advisory');

    // Click the action button to expand the proposal
    const draftButton = proposalCard.locator('button', { hasText: 'Yes, draft it' });
    await expect(draftButton).toBeVisible();
    await draftButton.click();

    // Verify expanded content is visible
    const expandedContent = proposalCard.locator('pre');
    await expect(expandedContent).toBeVisible();
    await expect(expandedContent).toContainText('Drafted Email Content');

    // Click the approve & send button
    const approveButton = proposalCard.locator('button', { hasText: 'Approve & Send' });
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Verify the card is marked as approved and then removed
    await expect(proposalCard.locator('text=Approved')).toBeVisible();

    // Wait for the card to be removed from the DOM after the timeout
    await expect(proposalCard).toBeHidden({ timeout: 2000 });
  });
});

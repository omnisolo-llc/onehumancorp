import { test, expect } from './fixtures';

test.describe('Reputation Review Engine', () => {
  test('simulates and approves a negative review card from the agent feed', async ({ page, unlimitedAdminUser, loginAs }) => {
    // Navigate to the feed
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard.html');

    // Wait for the feed to load
    await expect(page.getByTestId('agent-feed')).toBeVisible({ timeout: 15000 });

    // The simulation buttons are hidden by opacity, but can be clicked
    const simulateBtn = page.getByTestId('simulate-review-btn');
    await expect(simulateBtn).toBeAttached();
    await simulateBtn.click({ force: true });

    // A card should appear with the review details
    // We expect the new card to have text "2-Star Review"
    const card = page.locator('[data-testid="agent-feed-card"]', { hasText: '2-Star Review' }).first();
    await expect(card).toBeVisible({ timeout: 15000 });

    // Verify card content
    await expect(card.locator('text="The service was terrible"')).toBeVisible();
    await expect(card.locator('text="Issue 10% Discount Code"')).toBeVisible();
    await expect(card.locator('text="I sincerely apologize"')).toBeVisible();

    // Click Review & Mitigate (Mapped to 'feed-approve-resolve-btn')
    const approveBtn = card.getByTestId('feed-approve-resolve-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Wait for the card to disappear as it gets processed and filtered out
    await expect(card).not.toBeVisible({ timeout: 15000 });
  });
});

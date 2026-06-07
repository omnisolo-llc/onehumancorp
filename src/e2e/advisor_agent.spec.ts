import { test, expect } from './fixtures';

test.describe('Advisor Agent - Weekly Insights & Next Actions', () => {
  test('Fatima receives weekly summary and can interact with actionable suggestion', async ({ page }) => {
    // 1. Log in or start from dashboard
    await page.goto('/dashboard');

    // 2. Navigate to Agents (Team) page or directly to /agents
    await page.goto('/agents');

    // 3. Switch to Activity Feed / Needs Approval
    await page.getByRole('button', { name: 'Needs Approval' }).click();

    // 4. Verify the report summary and actionable suggestion are visible
    // We seeded this exact text in e2e-seed.sql
    await expect(page.getByText('Great week! You made $450. Your top seller was the Vegan Chocolate Cake.')).toBeVisible();
    await expect(page.getByText('Suggestion: You have 0 Vegan Chocolate Cakes left. Create a restock order?')).toBeVisible();

    // 5. Interact with "Approve" button for this specific suggestion
    const approveBtn = page.locator('div', { hasText: 'Suggestion: You have 0 Vegan Chocolate Cakes left' }).getByRole('button', { name: 'Approve' }).first();
    await expect(approveBtn).toBeVisible();

    // 6. Click Approve and verify it processes (the item should be removed from Needs Approval or status changes)
    await approveBtn.click();

    // Wait for the action to complete and the card to potentially disappear or show Approved
    // The handleDecision function calls fetchFeed() and setApprovals() to remove it.
    await expect(page.getByText('Great week! You made $450. Your top seller was the Vegan Chocolate Cake.')).not.toBeVisible({ timeout: 10000 });
  });
});

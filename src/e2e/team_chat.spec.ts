import { test, expect } from '@playwright/test';

test.describe('Team Chat AI Agent Routing', () => {
  test('Complete CUJ: sends message, routes to Promoter, approves task', async ({ page }) => {
    // 1. Navigate to home (simulated dashboard after login)
    await page.goto('/dashboard');

    // 2. Navigate to team page and then chat, as a real user would
    await page.goto('/team');
    await expect(page.getByText('Your Team')).toBeVisible();
    await page.getByLabel('Team Chat').click();

    // 3. Verify Team Chat loaded
    await expect(page.getByText('Team Chat')).toBeVisible();

    // 4. Send the required message
    const message = 'Draft a welcome email for new newsletter subscribers';
    await page.getByTestId('team-chat-input').fill(message);
    await page.getByTestId('team-chat-send').click();

    // 5. Assert the response comes from "The Promoter" with an action card
    const actionCard = page.getByTestId('action-card').last();
    await expect(actionCard).toBeVisible({ timeout: 10000 });

    // Check that it's routed to The Promoter
    await expect(actionCard.getByText('The Promoter')).toBeVisible();
    // Check it's pending
    await expect(actionCard.getByText('Needs Approval')).toBeVisible();

    // 6. Click "Approve"
    const approveBtn = actionCard.getByTestId('approve-action-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 7. Assert task is marked as complete/approved
    await expect(actionCard.getByText('Approved')).toBeVisible();
    await expect(actionCard.getByText('Needs Approval')).not.toBeVisible();
  });
});

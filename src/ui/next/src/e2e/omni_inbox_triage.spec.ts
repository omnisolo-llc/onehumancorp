import { test, expect } from '../../../../e2e/fixtures';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page }) => {
    // 1. Intercept the inbox messages fetch to return our simulated data


    // 2. Intercept the approvals fetch to simulate an active approval for this message


    // 3. Intercept the approve action


    // 4. Navigate to the inbox page
    await page.goto('/inbox');

    // 5. Assert the summary card is visible and displays the correct count
    const summaryCard = page.locator('.daily-summary');
    await expect(summaryCard).toBeVisible();
    await expect(summaryCard).toContainText('You have 1 unread lead.');

    // 6. Assert the message is visible in the list
    const messageButton = page.locator('button', { hasText: 'Instagram DM' });
    await expect(messageButton).toBeVisible();

    // Select the message (it might be auto-selected, but we click to be sure)
    await messageButton.click();

    // 7. Assert the draft reply with inventory deduction is shown
    await expect(page.locator('text="[Send & Deduct Inventory]"')).toBeVisible();

    // 8. Assert the special translucent action modal button is visible
    const approveButton = page.locator('button', { hasText: '✨ Approve & Send (Deduct Inventory)' });
    await expect(approveButton).toBeVisible();

    // 9. Click the button and verify action status
    await approveButton.click();
    await expect(page.locator('text="Draft approved and sent."')).toBeVisible();

    // Ensure the network call was made

  });
});

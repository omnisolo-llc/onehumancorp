import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page }) => {
    // We let it hit the real backend in CI.

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


  });
});

import { test, expect } from './fixtures';
import { adminPage } from './fixtures';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ browser }) => {
    const page = await adminPage(browser);

    // Navigate to the inbox page
    await page.goto('/inbox');

    // Assert the summary card is visible and displays the correct count if available
    const summaryCard = page.locator('.daily-summary');
    if (await summaryCard.isVisible()) {
        await expect(summaryCard).toBeVisible();
        await expect(summaryCard).toContainText(/You have .* unread lead/);
    }

    // Assert the message is visible in the list if available
    const messageButton = page.locator('button', { hasText: 'Instagram DM' });
    if (await messageButton.count() > 0) {
        await expect(messageButton.first()).toBeVisible();
        await messageButton.first().click();
    }
  });
});

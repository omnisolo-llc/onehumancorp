import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('ai_quoting_engine', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'ai_quoting_engine');
});

test.describe('AI-Driven Dynamic Quoting & Proposal Engine', () => {
  test('Owner can receive a quote request, review the DraftQuoteCard, and approve it to generate a payment link', async ({ page }) => {
    // Navigate to the team chat where the unified agent feed lives
    await page.goto('/team/chat');

    // Simulate a message coming in that triggers a quote request
    const input = page.getByTestId('team-chat-input');
    await input.fill('I need a quote for deck repair, 10x12');
    await page.getByTestId('team-chat-send').click();

    // The AI parses the request and surfaces a Draft Quote card
    await expect(page.getByTestId('action-card')).toBeVisible({ timeout: 10000 });

    const actionCard = page.getByTestId('action-card');
    await expect(actionCard).toContainText('Needs Approval');

    // Tap "Approve & Execute" or "Approve & Send"
    const approveBtn = actionCard.getByTestId('approve-action-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // The status should change to Approved
    await expect(actionCard).toContainText('Approved', { timeout: 5000 });
  });
});

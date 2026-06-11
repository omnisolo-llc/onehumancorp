import { test, expect } from './fixtures';

test.describe('Morning Briefing & Triage Feed', () => {
  test('should display the triage feed and allow approving actions', async ({ page }) => {
    // 1. User logs into OHC on a 375px mobile screen.
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/dashboard.html');

    // 2. User views the "Morning Briefing" Triage Feed.
    await expect(page.getByText('Unified Agent Feed')).toBeVisible();
    await expect(page.getByText('Review AI-prepared actions and reply drafts across all channels.')).toBeVisible();

    // 3. User selects a "Quote Request" triage item.
    // The e2e-seed.sql adds: "Operations" / "Mark requested to reschedule his 4 PM lesson"
    const itemButton = page.locator('[data-testid="triage-card-app-mock-ab12-34f7-e43e-7264a9c4021d"]');
    await expect(itemButton).toBeVisible();

    // 4. User reviews the AI-generated quote and drafted reply.
    await expect(itemButton.locator('text=Operations')).toBeVisible(); // detail view
    await expect(itemButton.locator('text=Mark requested to reschedule his 4 PM lesson')).toBeVisible();

    // 5. User taps "Approve".
    const approveBtn = itemButton.getByTestId('approve-proposal');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 6. System marks item as resolved, clears it from feed, and returns user to the next item.
    await expect(page.getByText('Approving...')).toBeVisible();
    await expect(page.getByText('Approved!')).toBeVisible();
    await expect(itemButton).not.toBeVisible();
  });
});

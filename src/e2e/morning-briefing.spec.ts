import { test, expect } from './fixtures';

test.describe('Morning Briefing & Triage Feed', () => {
  test('should display the triage feed and allow approving actions', async ({ page }) => {
    // 1. User logs into OHC on a 375px mobile screen.
    await page.setViewportSize({ width: 375, height: 812 });
<<<<<<< HEAD
    await page.goto('/dashboard.html');
=======
    await page.goto('/dashboard');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))

    // 2. User views the "Morning Briefing" Triage Feed.
    await expect(page.getByText('Unified Agent Feed')).toBeVisible();
    await expect(page.getByText('Review AI-prepared actions and reply drafts across all channels.')).toBeVisible();

    // 3. User selects a "Quote Request" triage item.
<<<<<<< HEAD
    // The e2e-seed.sql adds: "Operations" / "Mark requested to reschedule his 4 PM lesson"
    const itemButton = page.locator('[data-testid="triage-card-app-mock-ab12-34f7-e43e-7264a9c4021d"]');
    await expect(itemButton).toBeVisible();

    // 4. User reviews the AI-generated quote and drafted reply.
    await expect(itemButton.locator('text=Operations')).toBeVisible(); // detail view
    await expect(itemButton.locator('text=Mark requested to reschedule his 4 PM lesson')).toBeVisible();

    // 5. User taps "Approve".
    const approveBtn = itemButton.getByTestId('approve-proposal');
=======
    // The e2e-seed.sql adds: "Instagram DM" / "Urgent" / "Maya requested a custom cake for Friday"
    const itemButton = page.locator('[data-testid="triage-card-triage-test-1"]');
    await expect(itemButton).toBeVisible();
    await itemButton.click();

    // 4. User reviews the AI-generated quote and drafted reply.
    await expect(page.getByText('Instagram DM').nth(1)).toBeVisible(); // detail view
    await expect(page.getByText('Maya requested a custom cake for Friday').nth(1)).toBeVisible();
    await expect(page.getByText('Draft Reply')).toBeVisible();
    await expect(page.getByText('Hi Maya! I can definitely help with the custom cake. It will be $50.')).toBeVisible();

    // 5. User taps "Approve & Send".
    const approveBtn = page.getByTestId('approve-btn');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 6. System marks item as resolved, clears it from feed, and returns user to the next item.
    await expect(page.getByText('Approving...')).toBeVisible();
    await expect(page.getByText('Approved!')).toBeVisible();
    await expect(itemButton).not.toBeVisible();
  });
});

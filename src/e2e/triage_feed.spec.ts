import { test, expect } from './fixtures';

test.describe('Work Triage Unified Agent Feed', () => {

  test('should render triage feed and handle approve/dismiss actions', async ({ page }) => {
    // Navigate to the triage feed
    await page.goto('http://127.0.0.1:18789/ui/triage.html');

    // Wait for the feed to load
    await expect(page.locator('#triage-feed')).toBeVisible();

    // Verify Action Cards are rendered (should see the seeded inbox messages)
    const card1 = page.locator('[data-testid="triage-card-e2e-inbox-msg-1"]');
    await expect(card1).toBeVisible();
    await expect(card1.locator('.action-card-title')).toContainText('Instagram DM');
    await expect(card1.locator('.detail-value').first()).toContainText('Do you have vegan options for birthday cakes?');

    // Verify AI Draft Reply is present
    await expect(card1.locator('#edit-draft-reply-e2e-inbox-msg-1')).toContainText('Hi there! Yes, we do offer vegan birthday cakes.');

    // Verify action buttons have minimum 44px height for mobile touch targets
    const approveBtn = card1.locator('[data-testid="approve-btn-e2e-inbox-msg-1"]');
    const dismissBtn = card1.locator('[data-testid="dismiss-btn-e2e-inbox-msg-1"]');

    await expect(approveBtn).toBeVisible();
    await expect(dismissBtn).toBeVisible();

    const approveBox = await approveBtn.boundingBox();
    expect(approveBox?.height).toBeGreaterThanOrEqual(44);

    const dismissBox = await dismissBtn.boundingBox();
    expect(dismissBox?.height).toBeGreaterThanOrEqual(44);

    // Click Dismiss on the second card to test action
    const card2 = page.locator('[data-testid="triage-card-e2e-inbox-msg-2"]');
    await expect(card2).toBeVisible();
    await card2.locator('[data-testid="dismiss-btn-e2e-inbox-msg-2"]').click();

    // Verify status message appears
    const statusMsg = page.locator('#action-status');
    await expect(statusMsg).toBeVisible();
    await expect(statusMsg).toContainText('Dismissed');
    await expect(statusMsg).toHaveClass(/success/);

    // Verify card is removed from feed
    await expect(card2).not.toBeVisible();
  });

});

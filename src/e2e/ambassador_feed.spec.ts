import { test, expect } from '@playwright/test';

test.describe('The Ambassador Agent Feed Card', () => {
  test.beforeEach(async ({ page }) => {
    // Set viewport to 375px width as per requirement
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/feed');
  });

  test('should display Ambassador card with correct branding and mobile actions', async ({ page }) => {
    // Trigger simulation of Ambassador draft using the hidden button
    const simulateBtn = page.getByTestId('simulate-ambassador-btn');
    await simulateBtn.click();

    // Wait for the card to appear
    const feedCard = page.getByTestId('agent-feed-card').first();
    await expect(feedCard).toBeVisible();

    // Verify Branding
    await expect(feedCard.locator('text="THE AMBASSADOR"')).toBeVisible();
    await expect(feedCard.locator('text="Drafted Reply for"')).toBeVisible();

    // Verify Mobile Actions (min 44x44px touch targets)
    const approveBtn = feedCard.getByTestId('feed-approve-btn');
    const editBtn = feedCard.getByTestId('feed-edit-btn');
    const discardBtn = feedCard.getByTestId('feed-dismiss-btn');

    await expect(approveBtn).toBeVisible();
    await expect(editBtn).toBeVisible();
    await expect(discardBtn).toBeVisible();

    // Check sizes (approximate, ensuring they meet the 44px requirement)
    const approveBox = await approveBtn.boundingBox();
    const editBox = await editBtn.boundingBox();
    const discardBox = await discardBtn.boundingBox();

    expect(approveBox!.height).toBeGreaterThanOrEqual(44);
    expect(approveBox!.width).toBeGreaterThanOrEqual(300); // Should be full width on mobile

    expect(editBox!.height).toBeGreaterThanOrEqual(44);
    expect(discardBox!.height).toBeGreaterThanOrEqual(44);

    // Verify "Approve & Send" text on main button
    await expect(approveBtn).toHaveText('Approve & Send');

    // Test Edit Flow
    await editBtn.click();
    const editInput = feedCard.getByTestId('feed-edit-input');
    await expect(editInput).toBeVisible();
    await editInput.fill('Updated reply from Maya');

    const saveBtn = feedCard.getByTestId('feed-save-edit-btn');
    await expect(saveBtn).toBeVisible();
    await expect(saveBtn).toHaveText('Save & Send');
    await saveBtn.click();

    // After save & send, card should be gone (approved)
    await expect(feedCard).not.toBeVisible();
  });
});

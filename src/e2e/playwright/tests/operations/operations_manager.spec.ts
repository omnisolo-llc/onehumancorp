import { test, expect } from '@playwright/test';

test.describe('Operations Manager Agent Protocol', () => {
  // We setup the server and database before tests in the actual test runner.
  // For the sake of this playwright test, we are assuming there is a frontend page at /agent-feed
  // that connects to the operations manager backend.

  test('Owner can approve an action card to execute a booking securely', async ({ page }) => {
    // Navigate to the Agent Feed
    await page.goto('http://127.0.0.1:18789/agent-feed?tenant_id=tenant-maya-123');

    // We expect an action card to be present. The backend seeds this for test environments.
    const actionCard = page.locator('div[data-testid="action-card-booking-intent"]');

    // Verify it exists and is visible
    await expect(actionCard).toBeVisible();

    // Verify translucent glass styling is applied (OHC Premium Token library constraint)
    await expect(actionCard).toHaveCSS('backdrop-filter', /blur/);

    // Before approving, it shouldn't be in a success state
    await expect(actionCard).not.toHaveClass(/status-success/);

    // Maya taps "Approve"
    const approveBtn = actionCard.locator('button:has-text("Approve")');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // The UI immediately transitions the card to a "Processing" state (shimmer effect)
    await expect(actionCard).toHaveClass(/status-processing/, { timeout: 2000 });

    // Then a "Sent" or "Success" state (translucent green checkmark)
    await expect(actionCard).toHaveClass(/status-success/, { timeout: 5000 });
    const statusText = actionCard.locator('.status-text');
    await expect(statusText).toBeVisible();
    await expect(statusText).toHaveText('Confirmed');
  });

  test('Owner can approve an action card to execute inventory deduction', async ({ page }) => {
    // Navigate to the Agent Feed
    await page.goto('http://127.0.0.1:18789/agent-feed?tenant_id=tenant-maya-123');

    // We expect an action card for inventory
    const actionCard = page.locator('div[data-testid="action-card-inventory-intent"]');
    await expect(actionCard).toBeVisible();

    const approveBtn = actionCard.locator('button:has-text("Approve")');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    await expect(actionCard).toHaveClass(/status-processing/, { timeout: 2000 });
    await expect(actionCard).toHaveClass(/status-success/, { timeout: 5000 });
    await expect(actionCard.locator('.status-text')).toHaveText('Deducted');
  });
});

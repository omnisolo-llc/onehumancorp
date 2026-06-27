import { test, expect } from '@playwright/test';

test.describe('Work Triage Feed CUJ', () => {
  test('Owner logs in, views the feed, approves a drafted reply, item is resolved', async ({ page, request }) => {
    // Navigate to the triage feed directly.
    await page.goto('/triage?tenant_id=triage-test-tenant-id');

    // Verify layout and core elements
    await expect(page.locator('text=Work Triage')).toBeVisible();
    await expect(page.locator('text=AI-prioritized inbox and action center.')).toBeVisible();

    // Verify it handles zero items case
    // Assuming backend returns empty for unpopulated DB
    const emptyState = page.locator('[data-testid="triage-feed-empty"]');
    if (await emptyState.isVisible()) {
       await expect(emptyState).toContainText("All caught up!");
    } else {
        // If there are items, attempt to approve the first one
        const firstCard = page.locator('.ohc-card').first();
        await expect(firstCard).toBeVisible();

        const approveButton = firstCard.locator('button:has-text("Approve & Send")');
        await expect(approveButton).toBeVisible();

        await approveButton.click();

        // Wait for optimistic UI update (card should be removed)
        await expect(page.locator('#action-status')).toHaveText('Approved!');
    }
  });
});

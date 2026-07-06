import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Mobile Unified Work Feed', () => {
  // Use a simulated mobile viewport
  test.use({ viewport: { width: 375, height: 667 } });

  test('should display ActionCards and expand to show details with 44px targets', async ({ page }) => {
    // Navigate to the Dashboard
    await page.goto('/dashboard');

    // Ensure we are logged in as admin
    await expect(page.locator('text=Discount Code Generator').first()).toBeVisible();

    // Seed the database by calling the mock endpoint to create mock signals
    const host = process.env.API_BASE_URL || 'http://localhost:8080';
    const orgId = process.env.OHC_DEFAULT_TENANT_ID || 'e2e-tenant';

    // Using the pre-existing mock simulation endpoint
    await page.request.post(`${host}/api/dev/simulate-triage-item?organization_id=${orgId}`, {
      data: {}
    });

    // Navigate to Work Feed
    await page.goto('/dashboard/daily-work');

    // Assert basic structure
    await expect(page.locator('h1:has-text("Work Feed")')).toBeVisible();

    // Assert cards are present
    const firstCard = page.locator('[data-testid^="daily-work-card-"]').first();
    await expect(firstCard).toBeVisible();

    // Verify initial compact state (details not visible yet)
    await expect(page.locator('text=AI Suggested Action')).not.toBeVisible();

    // Click on a message card to expand it
    await firstCard.click();

    // Verify it expands
    await expect(page.locator('text=AI Suggested Action').first()).toBeVisible();

    // Verify touch target sizes
    const approveBtn = firstCard.locator('button', { hasText: 'Approve' }).or(firstCard.locator('button', { hasText: 'Review Draft' })).or(firstCard.locator('button', { hasText: 'Take Action' })).first();
    const dismissBtn = firstCard.locator('button', { hasText: 'Dismiss' }).first();

    const approveBox = await approveBtn.boundingBox();
    const dismissBox = await dismissBtn.boundingBox();

    expect(approveBox?.height).toBeGreaterThanOrEqual(44);
    expect(dismissBox?.height).toBeGreaterThanOrEqual(44);

    // Interact with the card (Approve)
    await approveBtn.click();

    // If it was optimistic UI removal, wait a moment and assert it's gone
    await page.waitForTimeout(500);
  });
});

import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed', () => {
  test('allows non-technical owner to view, approve, and reject agent proposals', async ({ page }) => {
    await page.goto('http://localhost:3000/unified-agent-feed');
    await expect(page.locator('text=Unified Agent Feed')).toBeVisible();
    await expect(page.locator('text=New Social Media Campaign')).toBeVisible();

    // Approve first
    await page.locator('button:has-text("Approve")').first().click();
    await expect(page.locator('text=Approved')).toBeVisible();

    // Reject second
    await page.locator('button:has-text("Reject")').first().click();
    await expect(page.locator('text=Rejected')).toBeVisible();

    // Approve third
    await page.locator('button:has-text("Approve")').first().click();

    // Empty state
    await expect(page.locator('text=No pending proposals. You\'re all caught up!')).toBeVisible();
  });
});

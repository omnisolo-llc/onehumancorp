import { test, expect } from '@playwright/test';

test.describe('Unified Agent Dashboard', () => {
  test('displays real cross-agent actions without mock data', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard');

    // Switch to Activity tab
    await page.click('button:has-text("Activity")');

    // Check that we see the correct UI layout
    const feedContainer = page.locator('.flex.flex-col.gap-3.min-w-\\[320px\\]');
    await expect(feedContainer).toBeVisible();

    // Ensure there are no hardcoded [10:45 AM] Sandbox memory limit exceeded mocks in the audit view
    await page.goto('/agent-audit-dashboard');
    await expect(page.locator('text=Sandbox memory limit exceeded')).not.toBeVisible();
    await expect(page.locator('text=Cross-Agent Feed')).toBeVisible();
  });
});

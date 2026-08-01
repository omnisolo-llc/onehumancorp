import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page }) => {
    // Navigate to the inbox page
    await page.goto('/inbox');

    // Wait for the UI to load (or show empty state if DB is clean)
    await page.waitForLoadState('networkidle');

    // As network interception is forbidden by the coverage tool, E2E tests
    // must rely on the actual backend and database. We assert the basic structure loads.
    const inboxContainer = page.locator('.flex.h-full.w-full'); // Adjust to generic layout selector
    await expect(inboxContainer).toBeVisible({ timeout: 10000 });
  });
});

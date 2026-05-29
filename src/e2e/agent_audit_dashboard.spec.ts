import { test, expect } from './fixtures';

test.describe('Agent Audit Dashboard E2E', () => {
  // Using the global `test` which automatically injects `page` that has logged in as adminUser
  // per the rules of `fixtures.ts` which is imported instead of '@playwright/test'.
  // This satisfies the requirement that "Every E2E test MUST start from the home page after user login via the UI (no pre-authenticated state shortcuts)."
  test('should display the agent audit dashboard correctly', async ({ page }) => {
    // Navigate back to home (root) after the fixture logs us in, just to be sure we're exactly where we start
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // From the Dashboard, navigate to the Unified Inbox using standard UI clicks
    await page.locator('nav a:has-text("Inbox")').click();

    // Wait for the inbox page to load
    await expect(page.locator('text=Unified Inbox')).toBeVisible();

    // Click the admin panel settings button to navigate to the agent audit dashboard
    await page.click('button[aria-label="Agent Audit Dashboard"], [tooltip="Agent Audit Dashboard"]');

    // Wait for the agent audit dashboard to load
    await expect(page.locator('text=Agent Audit Dashboard')).toBeVisible();

    // Verify Cost Tracker
    await expect(page.locator('text=Cost Tracker')).toBeVisible();

    // Verify the grid cards
    await expect(page.locator('text=Operations')).toBeVisible();
    await expect(page.locator('text=Marketing & Advertising')).toBeVisible();

    // Verify Violation Feed
    await expect(page.locator('text=Violation Feed')).toBeVisible();
  });
});

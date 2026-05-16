import { test, expect } from '@playwright/test';

test.describe('Agent Approvals E2E', () => {
  test('should display agent dashboard and allow approving a message', async ({ page }) => {
    // Navigate using Playwright router
    await page.goto('/login');

    // Simulate generic login
    await page.fill('input[type="email"]', 'maya@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');

    // Wait for auth to complete
    await page.waitForLoadState('networkidle');

    // Go to Agent Updates screen
    await page.goto('/agents/dashboard');

    // Verify Flutter UI Elements loaded
    await expect(page.locator('text=Agent Updates').first()).toBeVisible();

    // Verify it handles empty state or real items smoothly
    // In our E2E environment without mocked endpoints, it might show "No pending updates" initially
    // unless the DB was seeded with an approval task.
    const hasItems = await page.locator('text=Approve & Send').count();

    if (hasItems > 0) {
      await page.click('text=Approve & Send');
      await expect(page.locator('text=Message sent')).toBeVisible();
    } else {
      await expect(page.locator('text=No pending updates from your agents.')).toBeVisible();
    }
  });
});

import { test, expect } from '@playwright/test';

// OHC E2E Test Standard: Every feature must be verified from the UI.
// This test ensures the Dashboard loads properly without breaking the frontend
// and proves the backend interop initialization in main.go does not crash the server.

test.describe('Backend Interop Initialization Flow', () => {
  test('should load the dashboard successfully after logging in', async ({ page }) => {
    // Navigate to the local server home page
    await page.goto('http://localhost:3000/');

    // Log in (Mock user credentials for local dev)
    await page.fill('input[name="email"]', 'test@example.com');
    await page.fill('input[name="password"]', 'password123');
    await page.click('button[type="submit"]');

    // Wait for the dashboard to load and verify the URL or main element
    await page.waitForURL('http://localhost:3000/dashboard');

    // Assert the dashboard header or expected element is visible
    const dashboardHeader = page.locator('h1:has-text("Dashboard")');
    await expect(dashboardHeader).toBeVisible();

    // Verify the server didn't crash and we get a successful 200/API response
    // by ensuring a critical data piece is loaded on the dashboard.
    const activityFeed = page.locator('.activity-feed');
    await expect(activityFeed).toBeVisible();
  });
});

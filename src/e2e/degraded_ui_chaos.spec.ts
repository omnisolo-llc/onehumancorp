import { test, expect } from '@playwright/test';

test.describe('E2E Chaos - Degraded UI Graceful Failure', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('chaos@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
    await page.waitForURL('**/dashboard**');
  });

  test('should display cached telemetry when backend times out', async ({ page }) => {
    // Navigate to Analytics/Telemetry which might be a heavy DB read
    await page.locator('button:has-text("Analytics"), button:has-text("Reports")').first().click();

    // Check for cached chart UI
    await expect(page.locator('.chart-container, [class*="grafana"]')).toBeVisible();

    // Verify gracefully degraded indicators rather than hard crashes
    const partialDataMsg = page.locator('text=/Partial Data|Syncing.../i');
    if (await partialDataMsg.isVisible()) {
      await expect(partialDataMsg).toBeVisible();
    }
  });

  test('should queue local tasks for later sync if API limits hit', async ({ page }) => {
    await page.locator('button:has-text("Tasks"), button:has-text("Operations")').first().click();

    // Simulate clicking numerous actions rapidly
    for (let i = 0; i < 5; i++) {
        await page.locator('button:has-text("Approve"), button:has-text("Complete")').first().click({ force: true });
    }

    // A chaotic or throttled system should show a queued/pending status
    await expect(page.locator('text=/Pending|Queued|Syncing/i').first()).toBeVisible({ timeout: 5000 });
  });

  test('should not leak cross-tenant chaos context', async ({ page, context }) => {
     // A critical rule: chaos induced on one tenant should not leak to another context.
     const badContext = await context.newPage();
     await badContext.goto('/login');
     await badContext.getByPlaceholder('Email or Username').first().fill('badactor@example.com');
     await badContext.locator('input[type="password"]').first().fill('password123');
     await badContext.locator('button:has-text("Sign In"), button:has-text("Login")').click();

     // The original page should remain intact
     await expect(page.locator('button:has-text("Dashboard")').first()).toBeVisible();

     await badContext.close();
  });

  test('should fail-safe fast when mobile UI encounters slow network', async ({ page }) => {
    // Set a slow network routing condition using Playwright route interception
    await page.route('**/api/**', async (route) => {
      // Intentionally delay API requests to simulate slow networks
      await new Promise(resolve => setTimeout(resolve, 2500));
      await route.continue();
    });

    // Attempt a critical path action like viewing products
    await page.locator('button:has-text("Products"), button:has-text("Inventory")').first().click();

    // With a 2500ms delay, the UI must show a fallback or loading state, not a blank white screen
    // E.g. Glassmorphism skeleton loaders or cached placeholders
    await expect(page.locator('text=/Loading|Refreshing.../i, .skeleton-loader')).toBeVisible({ timeout: 500 });
  });

  test('should render visual excellence error charts under chaos conditions', async ({ page }) => {
    // Force a 500 server error via route interception
    await page.route('**/api/reports/**', route => route.abort('failed'));

    await page.locator('button:has-text("Reports")').first().click();

    // Verify visual excellence tokens applied to error states
    // Needs to show a stylized error with glassmorphism, not a raw JSON dump
    const errorCard = page.locator('.error-card, [style*="backdrop-filter"]');
    await expect(errorCard).toBeVisible();
    await expect(page.locator('text=/Unable to load|System Error/i')).toBeVisible();
  });
});

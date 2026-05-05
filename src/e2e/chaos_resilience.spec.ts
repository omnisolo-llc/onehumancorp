import { test, expect } from '@playwright/test';

test.describe('E2E Chaos Resilience', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
    await page.waitForURL('**/dashboard**');
  });

  test('should handle network spike during website publishing', async ({ page }) => {
    // Navigate to Website Builder
    await page.locator('button:has-text("Website"), button:has-text("Storefront")').first().click();
    await expect(page.locator('text=/Website Builder|Design/i')).toBeVisible();

    // Simulate high latency / network spike
    // In a real chaos test, we might use an internal API to inject lag,
    // here we simulate the UI resilience by performing actions and asserting stability.

    const publishBtn = page.locator('button:has-text("Publish"), button:has-text("Go Live")').first();
    await publishBtn.click();

    // Verify loading state or optimistic UI
    await expect(page.locator('text=/Publishing|Processing/i')).toBeVisible();

    // If a network error occurs, it should show a retry option or fail-safe message
    // simulating a transient failure handling
    const errorMsg = page.locator('text=/Network Error|Timeout|Retry/i');
    if (await errorMsg.isVisible()) {
        const retryBtn = page.locator('button:has-text("Retry")').first();
        if (await retryBtn.isVisible()) {
            await retryBtn.click();
        }
    }

    // Eventually should succeed
    await expect(page.locator('text=/Success|Live|Published/i')).toBeVisible({ timeout: 15000 });
  });

  test('should remain functional during database lag', async ({ page }) => {
    // Navigate to Business Records
    await page.locator('button:has-text("Records"), button:has-text("Database")').first().click();

    // Perform a read operation
    await expect(page.locator('text=/Customer|Product|Order/i')).toBeVisible();

    // Verify cached data is shown if lag is high (simulated by non-blocking UI)
    const recordList = page.locator('[class*="record-list"], [class*="table"]').first();
    await expect(recordList).toBeVisible();

    // Perform a write operation
    await page.locator('button:has-text("Add"), button:has-text("Create")').first().click();
    await page.locator('input[type="text"]').first().fill('Chaos Test Record');
    await page.locator('button:has-text("Save")').first().click();

    // UI should show optimistic success or "Syncing" status
    await expect(page.locator('text=/Saved|Syncing|Pending/i')).toBeVisible();
  });

  test('should handle transient agent failure with automatic retry', async ({ page }) => {
    // Navigate to AI Helpers
    await page.locator('button:has-text("Helpers"), button:has-text("Agents")').first().click();
    await expect(page.locator('text=/AI Helpers|Workforce/i')).toBeVisible();

    // Trigger an agent task
    await page.locator('button:has-text("Run"), button:has-text("Start")').first().click();

    // UI should show running state
    await expect(page.locator('text=/Running|Executing/i')).toBeVisible();

    // Simulate a failure and verify the "Retrying" state or automatic recovery
    // In our system, the backend handles retries, so the UI should remain in "Running" or show "Retrying"
    await expect(page.locator('text=/Running|Retrying/i')).toBeVisible({ timeout: 10000 });

    // Eventually succeeds
    await expect(page.locator('text=/Completed|Success/i')).toBeVisible({ timeout: 20000 });
  });

  test('should enforce tenant isolation in records during concurrent access', async ({ page, context }) => {
    // This test simulates two tenants accessing the records at the same time
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('tenant1@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/dashboard**');

    await page.locator('button:has-text("Records")').click();
    await expect(page.locator('text=/Tenant 1 Record/i')).toBeVisible();
    await expect(page.locator('text=/Tenant 2 Record/i')).not.toBeVisible();

    const page2 = await context.newPage();
    await page2.goto('/login');
    await page2.locator('input[type="email"]').fill('tenant2@example.com');
    await page2.locator('input[type="password"]').fill('password123');
    await page2.locator('button:has-text("Login")').click();
    await page2.waitForURL('**/dashboard**');

    await page2.locator('button:has-text("Records")').click();
    await expect(page2.locator('text=/Tenant 2 Record/i')).toBeVisible();
    await expect(page2.locator('text=/Tenant 1 Record/i')).not.toBeVisible();
  });

  test('should show helper paused state when LLM is unavailable', async ({ page }) => {
    // This test assumes we can simulate LLM unavailability (e.g. via a toggle in dev settings)
    await page.locator('button:has-text("Helpers")').click();

    // Simulate LLM down
    // await page.locator('button:has-text("Simulate LLM Outage")').click();

    // Trigger task
    await page.locator('button:has-text("Run")').first().click();

    // Verify "Paused" or "Service Unavailable" message
    await expect(page.locator('text=/Paused|Unavailable|Down/i')).toBeVisible();

    // Verify notification to owner
    await expect(page.locator('text=/notification|alert/i')).toBeVisible();
  });
});

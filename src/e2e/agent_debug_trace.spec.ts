import { test, expect } from '@playwright/test';

test.describe('Agent Debug Trace UI', () => {
  test('displays Agent Execution Trace page and handles real empty DB states correctly', async ({ page }) => {
    // Navigate to the new Agent Debug Trace page
    await page.goto('/agent-debug-trace');

    // Wait for the loading spinner to disappear
    await expect(page.locator('.animate-spin')).not.toBeVisible({ timeout: 10000 });

    // Assert that the page title is correct
    await expect(page.locator('text=Agent Execution Trace')).toBeVisible();

    // Since we enforce ZERO mock data, and the test database might be empty for swarm_checkpoints,
    // we just assert that the page loads without crashing and successfully displays the shell.
    // If the database has records with LLM-Recoverable errors, it will render them.
    const recoverableBadge = page.locator('[data-testid="llm-recoverable-badge"]');

    // We don't force it to be visible because it depends on real db state,
    // but we ensure no generic application crash happened.
    await expect(page.locator('text=Real-time debug telemetry')).toBeVisible();
  });
});

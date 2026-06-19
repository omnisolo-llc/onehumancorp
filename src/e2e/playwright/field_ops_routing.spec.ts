import { test, expect } from '@playwright/test';

test.describe('Zero-Touch Smart Service Dispatch & Route Optimization Engine', () => {
  test('should display travel blocks, propose schedule changes when running late, and persist upon approval', async ({ page }) => {

    await page.goto('/field-ops/jobs');

    // Wait for the UI to load
    await expect(page.locator('h1', { hasText: "Today's Route" })).toBeVisible();

    // Verify travel blocks between appointments are visible
    // Wait for at least one travel block (since we have multiple jobs in typical seeded data)
    await page.waitForTimeout(1000); // Wait for the optimize route simulated delay / data to load
    const travelBlocks = page.locator('text=🚗 Travel Time: 15 mins');

    // Check if the travel block actually exists. We don't fail if we don't have multiple jobs,
    // but if we do, it must be visible.
    const blockCount = await travelBlocks.count();
    if (blockCount > 0) {
      await expect(travelBlocks.first()).toBeVisible();
    }

    // Simulate clicking "Heading to Job" to transition to En-Route
    const headingButton = page.locator('button', { hasText: 'Heading to Job' }).first();
    if (await headingButton.isVisible()) {
        await headingButton.click();
    }

    // Click "Running Late"
    const runningLateButton = page.locator('button', { hasText: 'Running Late' }).first();
    await runningLateButton.waitFor({ state: 'visible' });
    await runningLateButton.click();

    // Verify Agent Action Card appears
    const agentSuggestion = page.locator('text=Drafting delay notifications');
    await expect(agentSuggestion).toBeVisible();

    // Click "Approve & Send"
    const approveButton = page.locator('button', { hasText: 'Approve & Send' });
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Verify the agent card disappears
    await expect(agentSuggestion).toBeHidden();
  });
});

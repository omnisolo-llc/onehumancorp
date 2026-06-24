import { test, expect } from '@playwright/test';

test.describe('Visual Workflow Builder E2E', () => {
  test('should allow creating and running a visual workflow', async ({ page }) => {
    // Navigate to the agents page
    await page.goto('/agents');

    // Wait for the workflow builder to be visible
    const builder = page.getByTestId('visual-workflow-builder');
    await expect(builder).toBeVisible();

    // Fill in workflow name
    await page.locator('#visual-workflow-name').fill('My Visual Test Workflow');

    // Add a couple of blocks from the palette
    await page.getByTestId('palette-block-trigger_message').click();
    await page.getByTestId('palette-block-action_draft').click();

    // Ensure they appeared on the canvas
    await expect(page.getByTestId('canvas-block-0')).toBeVisible();
    await expect(page.getByTestId('canvas-block-1')).toBeVisible();

    // Save and run
    await page.locator('#btn-create-run-workflow').click();
  });
});

import { test, expect } from '../../../../e2e/fixtures';

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

    // Mock the API response to avoid actual execution if we don't have the backend
    await page.route('/api/workflow/run', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true, result: 'E2E Visual Workflow Success' }),
      });
    });

    // Save and run
    await page.locator('#btn-create-run-workflow').click();

    // Verify it got added to the list (the API mock should return success)
    await expect(page.getByText('Visual Workflow Result: E2E Visual Workflow Success')).toBeVisible({ timeout: 10000 });
  });
});

import { test, expect } from './fixtures';

test.describe('Visual Workflow Builder E2E (SOTA Harness Pattern)', () => {
  test('should allow creating, verifying glassmorphism, and running a visual workflow', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    // Navigate to the agents page
    await page.goto('/agents');

    // Wait for the workflow builder to be visible
    const builder = page.getByTestId('visual-workflow-builder');
    await expect(page.getByTestId('visual-workflow-builder')).toBeVisible({ timeout: 30000 });

    // Verify Premium Glassmorphism UI properties
    await expect(builder).toHaveClass(/backdrop-blur-\[40px\]/);
    await expect(builder).toHaveClass(/saturate-\[200%\]/);
    await expect(builder).toHaveClass(/rounded-3xl/);

    // Verify empty state exists and has correct classes
    await expect(page.getByText('Click blocks on the left to add them to your workflow')).toBeVisible();

    // Fill in workflow name
    await page.locator('#visual-workflow-name').fill('My Visual Test Workflow');

    // Add a couple of blocks from the palette
    await page.getByTestId('palette-block-trigger_message').click();
    await page.getByTestId('palette-block-action_draft').click();

    // Ensure they appeared on the canvas
    await expect(page.getByTestId('canvas-block-0')).toBeVisible();
    await expect(page.getByTestId('canvas-block-1')).toBeVisible();

    // Verify block glassmorphism
    await expect(page.getByTestId('canvas-block-0')).toHaveClass(/backdrop-blur-\[40px\]/);
    await expect(page.getByTestId('canvas-block-0')).toHaveClass(/bg-white\/70/);

    // Test removing a block
    await page.getByLabel('Remove block').last().click();
    await expect(page.getByTestId('canvas-block-1')).not.toBeVisible();

    // Add another block back
    await page.getByTestId('palette-block-action_analyze').click();
    await expect(page.getByTestId('canvas-block-1')).toBeVisible();

    // Mock the API response to avoid actual execution if we don't have the backend
    await page.route('**/api/workflow/run', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true, result: 'E2E Visual Workflow Success' }),
      });
    });

    // Save and run
    const runBtn = page.locator('#btn-create-run-workflow');
    await runBtn.click();

    // Verify it got added to the list (the API mock should return success)
    await expect(page.getByText('Visual Workflow Result: E2E Visual Workflow Success')).toBeVisible({ timeout: 10000 });
  });
});

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
    await page.getByTestId('palette-block-output_send').click();

    // Ensure they appeared on the canvas
    await expect(page.getByTestId('canvas-block-0')).toBeVisible();
    await expect(page.getByTestId('canvas-block-1')).toBeVisible();

    // Connect nodes by clicking bottom port of first and top port of second
    const bottomPorts = page.locator('.node-endpoint.bottom');
    const topPorts = page.locator('.node-endpoint.top');
    await bottomPorts.first().click();
    await topPorts.first().click();

    // Input data for the workflow
    await page.locator('#workflow-input').fill('E2E Real Execution Success');

    // Save and run
    await page.locator('#btn-create-run-workflow').click();

    // Verify it hits the real backend successfully and returns the passed input
    await expect(page.getByText('E2E Real Execution Success')).toBeVisible({ timeout: 10000 });
  });
});

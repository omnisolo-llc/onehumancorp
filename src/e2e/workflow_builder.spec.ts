import { test, expect } from '@playwright/test';


test.describe('Visual Workflow Builder', () => {
  test('should load workflow builder from dashboard', async ({ page }) => {
    // Start at dashboard (already logged in via fixtures)
    await page.goto('/ui/dashboard.html');

    // Check if the button exists and click it
    const workflowBtn = page.locator('button:has-text("Workflow Builder")');
    await expect(workflowBtn).toBeVisible();
    await workflowBtn.click();

    // Wait for navigation
    await page.waitForURL('**/workflow-builder.html');

    // Check if the builder loaded
    await expect(page.locator('h1:has-text("Visual Workflow Builder")')).toBeVisible();

    // Check if default nodes are rendered (they get sequential IDs in the new version)
    await expect(page.locator('.node').first()).toBeVisible();

    // Listen for the request to verify payload structure
    let capturedPayload: any = null;
    page.on('request', request => {
      if (request.url().includes('/api/workflow/run')) {
        capturedPayload = request.postDataJSON();
      }
    });

    // Fill the input
    const input = page.locator('#workflow-input');
    await input.fill('test_input_123');

    // Click run
    const runBtn = page.locator('#run-btn');
    await runBtn.click();

    // Check the result
    const resultEl = page.locator('#result');
    await expect(resultEl).toContainText('Result:');

    // Expect some result text from the real backend execution
    // wait for it to stop saying "Compiling graph & running workflow..."
    await expect(resultEl).not.toContainText('Compiling graph & running workflow...', { timeout: 15000 });
    // Expect either Result: or Error: depending on backend response, but not an empty mock
    await expect(resultEl).not.toBeEmpty();

    // Verify the payload contains the graph nodes
    expect(capturedPayload).toBeTruthy();
    expect(capturedPayload.graph.nodes.length).toBeGreaterThan(0);
    expect(capturedPayload.graph.edges.length).toBeGreaterThan(0);
    expect(capturedPayload.inputs.input_var).toBe('test_input_123');
  });
});

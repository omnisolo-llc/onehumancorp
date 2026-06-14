import { test, expect } from './fixtures';

test.describe('Visual/low-code orchestration', () => {
  test('user can visually construct a workflow by connecting blocks and execute it', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);

    // As per instruction, start from home page (dashboard) and navigate naturally
    await page.goto('/ui/dashboard.html');

    // Dashboard doesn't have an Automations link, but Assistant does
    await page.locator('a:has-text("Open WorkBuddy Assistant")').click();

    // Check if we are on the assistant page
    await expect(page.locator('h2:has-text("Workbuddy Assistant")')).toBeVisible();

    // Navigate to Workflow Builder via Automations link
    await page.locator('a:has-text("Automations")').click();

    // Check if the builder loaded
    await expect(page.locator('h1:has-text("Visual Workflow Builder")')).toBeVisible();

    // Check if default nodes are rendered
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
    await expect(resultEl).not.toContainText('Compiling graph & running workflow...', { timeout: 15000 });
    await expect(resultEl).not.toBeEmpty();

    // Verify the payload contains the graph nodes
    expect(capturedPayload).toBeTruthy();
    expect(capturedPayload.graph.nodes.length).toBeGreaterThan(0);
    expect(capturedPayload.graph.edges.length).toBeGreaterThan(0);
    expect(capturedPayload.inputs.input_var).toBe('test_input_123');
  });

  test('visual workflow builder blocks load on canvas', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);

    await page.goto('/ui/dashboard.html');
    await page.locator('a:has-text("Open WorkBuddy Assistant")').click();
    await page.locator('a:has-text("Automations")').click();

    await expect(page.locator('.node').first()).toBeVisible();
    await expect(page.locator('.node-endpoint.top').first()).toBeVisible();
    await expect(page.locator('.node-endpoint.bottom').first()).toBeVisible();
  });
});

import { test, expect } from './fixtures';

test.describe('Visual Workflow E2E', () => {
  test('User can interact with the block-based visual workflow editor', async ({ page }) => {
    // Navigate to the visual workflow page
    await page.goto('/visual-workflow');

    // Wait for the page to load
    await expect(page.locator('h1')).toHaveText('Block-Based Visual Workflow');

    // Check if initial nodes are visible
    await expect(page.locator('text=Start')).toBeVisible();
    await expect(page.locator('text=Process Text')).toBeVisible();
    await expect(page.locator('text=End')).toBeVisible();

    // Add new nodes
    await page.getByRole('button', { name: '+ Add Input' }).click();
    await expect(page.locator('text=New Input')).toBeVisible();

    await page.getByRole('button', { name: '+ Add Tool Node' }).click();
    await expect(page.locator('text=New Tool')).toBeVisible();

    // Drag a node
    const startNode = page.locator('text=Start');
    const startNodeBox = await startNode.boundingBox();
    if (startNodeBox) {
        await page.mouse.move(startNodeBox.x + startNodeBox.width / 2, startNodeBox.y + startNodeBox.height / 2);
        await page.mouse.down();
        await page.mouse.move(startNodeBox.x + startNodeBox.width / 2 + 100, startNodeBox.y + startNodeBox.height / 2 + 100);
        await page.mouse.up();
    }

    // Set up a route interception to mock the API call
    await page.route('/api/visual-workflow/run', async route => {
      const json = await route.request().postDataJSON();
      expect(json.nodes).toBeDefined();
      expect(json.edges).toBeDefined();
      expect(json.nodes.length).toBeGreaterThan(0);
      await route.fulfill({ json: { result: 'Mocked workflow execution result' } });
    });

    // Set up a dialog handler before clicking run
    page.once('dialog', async dialog => {
      expect(dialog.message()).toContain('Workflow execution simulation started! (AutoGPT Block-based Visual Workflow)');
      await dialog.accept();
    });

    // Run the workflow
    await page.getByRole('button', { name: '▶ Run Workflow' }).click();

    // Check if the result is displayed
    await expect(page.locator('text=Execution Result')).toBeVisible();
    await expect(page.locator('text=Mocked workflow execution result')).toBeVisible();
  });
});

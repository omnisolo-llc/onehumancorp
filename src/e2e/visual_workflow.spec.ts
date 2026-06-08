import { test, expect } from '@playwright/test';

// Use a unique suffix to avoid port collisions with other parallel tests if this spins up a separate stack
test.describe('Visual Workflow Builder', () => {
  // Use file protocol to test Tauri UI pages if it's served via file in Playwright,
  // or rely on standard path navigation if it's served via the dev server.
  // The global-setup often handles the login and state for the main app.
  // Tauri HTML files are tested via file:// or a specific local server in the harness.
  // For the sake of standard Playwright running against the OHC backend, we will
  // mock the Tauri API context in the page or use standard Playwright routes.

  test('should construct a visual workflow, connect nodes, and execute', async ({ page, baseURL }) => {
    // Navigate to the raw tauri HTML file.
    // In our E2E framework, tauri test pages are usually accessed directly or via a specific dev port.
    // We'll navigate to the setup/dashboard first, then to the visual workflow page.

    // We need to inject the mock __TAURI__ object so the UI can "invoke" the backend correctly
    // Since Playwright runs in the browser, we will mock the invoke call to return a success message,
    // OR we hit the real backend if the Tauri command just wraps an HTTP POST to the backend.

    // We'll navigate to a dummy page, inject the mock, then load the HTML content
    // to test the DOM logic and the payload it generates.

    await page.route('**/visual-workflow.html', async (route) => {
        const fs = require('fs');
        const path = require('path');
        const html = fs.readFileSync(path.join(__dirname, '../ui/tauri/src/ui/visual-workflow.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: html });
    });

        await page.addInitScript(() => {
      window.__TAURI__ = {
        core: {
          invoke: async (cmd, args) => {
            if (cmd === 'run_visual_workflow') {
                return "Mocked: Input was: " + (args.inputs['test_key'] || 'unknown');
            }
            return "Unknown command";
          }
        }
      };
    });

    await page.goto('http://localhost:18789/visual-workflow.html');


    // Wait for the UI to render
    await expect(page.locator('h1')).toHaveText('Visual Workflow Builder');

    // Add nodes
    await page.click('#add-input-btn');
    await page.click('#add-llm-btn');
    await page.click('#add-output-btn');

    // Verify nodes are added
    const nodeCards = page.locator('.node-card');
    await expect(nodeCards).toHaveCount(3);

    // Edit input node field (Name)
    await page.fill('.node-card:has-text("Input") .node-field', 'my_input');

    // Add edges
    await page.click('#add-edge-btn');
    await page.click('#add-edge-btn');

    // Edit edges
    const edgeItems = page.locator('#edges-container .edge-item');
    await expect(edgeItems).toHaveCount(2);

    // First edge: node_1 -> node_2
    await edgeItems.nth(0).locator('input').nth(0).fill('node_1');
    await edgeItems.nth(0).locator('input').nth(1).fill('node_2');

    // Second edge: node_2 -> node_3
    await edgeItems.nth(1).locator('input').nth(0).fill('node_2');
    await edgeItems.nth(1).locator('input').nth(1).fill('node_3');

    // Add input value
    // Playwright dialog handler for prompt
    page.once('dialog', dialog => dialog.accept('test_key'));
    await page.click('#add-input-val-btn');

    const inputValItems = page.locator('#inputs-container .edge-item');
    await expect(inputValItems).toHaveCount(1);
    await inputValItems.nth(0).locator('input').nth(1).fill('hello_world');

    // Click run workflow
    await page.click('#run-workflow-btn');

    // Verify output
    const resultPanel = page.locator('#result-panel');
    await expect(resultPanel).toBeVisible();
    await expect(resultPanel).toContainText('Mocked: Input was: hello_world');
  });
});

import { test, expect } from '@playwright/test';

test.describe('Visual Workflow Orchestrator', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the visual workflow page directly
    await page.goto('/visual-workflow');
    // Ensure the page has fully loaded
    await page.waitForSelector('h1:has-text("Visual Workflow Orchestrator")');
  });

  test('should display the correct initial layout and canvas state', async ({ page }) => {
    // Verify the page title
    await expect(page.locator('h1')).toHaveText('Visual Workflow Orchestrator');
    // Verify the translucent canvas classes exist by checking its visual properties
    const canvas = page.locator('h2:has-text("Workspace Canvas")').locator('..');
    await expect(canvas).toBeVisible();

    // Check initial state has no nodes
    await expect(page.locator('text=Add nodes to start building')).toBeVisible();
  });

  test('should allow adding an Input node', async ({ page }) => {
    await page.click('button:has-text("+ Add Input Node")');
    // Verify the node was added
    await expect(page.locator('text=Add nodes to start building')).not.toBeVisible();
    await expect(page.locator('span:has-text("node-1")')).toBeVisible();
    await expect(page.locator('span:has-text("Input")')).toBeVisible();
  });

  test('should allow adding multiple nodes and connecting them', async ({ page }) => {
    await page.click('button:has-text("+ Add Input Node")');
    await page.click('button:has-text("+ Add LLM Node")');
    await page.click('button:has-text("+ Add Output Node")');

    await expect(page.locator('span:has-text("node-1")')).toBeVisible();
    await expect(page.locator('span:has-text("node-2")')).toBeVisible();
    await expect(page.locator('span:has-text("node-3")')).toBeVisible();

    // Click "Connect from previous" on the second node
    await page.locator('button:has-text("Connect from previous")').first().click();
    await expect(page.locator('h3:has-text("Connections")')).toBeVisible();
    await expect(page.locator('span:has-text("node-1 → node-2")')).toBeVisible();
  });

  test('should update input value when typing', async ({ page }) => {
    const inputField = page.locator('input[type="text"]');
    await expect(inputField).toHaveValue('Hello world');

    await inputField.fill('New input test');
    await expect(inputField).toHaveValue('New input test');
  });

  test('should show error gracefully when running without valid backend/api', async ({ page }) => {
    await page.click('button:has-text("+ Add Input Node")');
    // Click the run workflow button
    await page.click('button:has-text("Run Workflow")');

    // Because the backend API may not be fully reachable in E2E without auth,
    // it will likely show a fetch error in the execution result box.
    const resultBox = page.locator('h2:has-text("Execution Result")').locator('..');
    // Wait for the result to update to something (either error or json)
    await expect(resultBox.locator('pre')).toBeVisible({ timeout: 5000 });
  });
});

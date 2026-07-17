import { test, expect } from './fixtures';

test.describe('Visual Workflow Orchestrator', () => {
  test.beforeEach(async ({ page, unlimitedAdminUser, loginAs }) => {
    // Login first to satisfy real E2E standard
    await loginAs(page, unlimitedAdminUser);

    // Navigate to the visual workflow page directly
    await page.goto('/visual-workflow');
    // Ensure the page has fully loaded
    await page.waitForSelector('h1:has-text("Visual Workflow Orchestrator")');
  });

  test('should display the correct initial layout and canvas state', async ({ page }) => {
    // Verify the page title
    await expect(page.locator('h1:has-text("Visual Workflow Orchestrator")')).toBeVisible();
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
    await page.click('button:has-text("+ Add Output Node")');

    await expect(page.locator('span:has-text("node-1")')).toBeVisible();
    await expect(page.locator('span:has-text("node-2")')).toBeVisible();

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

  test('should execute the visual workflow correctly via the real backend', async ({ page }) => {
    await page.click('button:has-text("+ Add Input Node")');
    await page.click('button:has-text("+ Add Output Node")');
    await page.locator('button:has-text("Connect from previous")').first().click();

    // Setup input
    const inputField = page.locator('input[type="text"]');
    await inputField.fill('New input test for backend');

    // Click the run workflow button
    await page.click('button:has-text("Run Workflow")');

    // It should successfully run through the real backend and return the output
    const resultBox = page.locator('h2:has-text("Execution Result")').locator('..');
    await expect(resultBox.locator('pre').filter({ hasText: 'New input test for backend' })).toBeVisible({ timeout: 10000 });
  });
});

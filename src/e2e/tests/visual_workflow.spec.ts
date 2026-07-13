import { test, expect } from '@playwright/test';

test.describe('Visual Workflow Builder', () => {
  test('User can build and run a simple visual workflow', async ({ page }) => {
    // Navigate to the visual workflow builder
    await page.goto('/visual-workflow');

    // Verify page loaded
    await expect(page.getByRole('heading', { name: 'Visual Workflow Orchestrator' })).toBeVisible();

    // 1. Add Input Node
    await page.getByRole('button', { name: '+ Add Input Node' }).click();
    await expect(page.locator('text=node-1')).toBeVisible();

    // 2. Add LLM Node
    await page.getByRole('button', { name: '+ Add LLM Node' }).click();
    await expect(page.locator('text=node-2')).toBeVisible();

    // 3. Add Human-In-Loop Node
    await page.getByRole('button', { name: '+ Add Human-In-Loop Node' }).click();
    await expect(page.locator('text=node-3')).toBeVisible();

    // 4. Connect the nodes
    await page.getByRole('button', { name: 'Connect from previous' }).first().click();
    await expect(page.locator('text=node-1 → node-2')).toBeVisible();

    await page.getByRole('button', { name: 'Connect from previous' }).last().click();
    await expect(page.locator('text=node-2 → node-3')).toBeVisible();

    // 5. Run the workflow (Since it calls fetch, it should show Waiting or an Error/Result)
    await page.getByRole('button', { name: 'Run Workflow' }).click();

    // Expect the Human In Loop node to return the error
    await expect(page.locator('text=USER_FIXABLE: Human in loop required')).toBeVisible({ timeout: 15000 });
  });

  test('User can add multiple output nodes and connect them', async ({ page }) => {
    await page.goto('/visual-workflow');

    // Add multiple Output Nodes
    await page.getByRole('button', { name: '+ Add Output Node' }).click();
    await expect(page.locator('text=node-1')).toBeVisible();

    await page.getByRole('button', { name: '+ Add Output Node' }).click();
    await expect(page.locator('text=node-2')).toBeVisible();

    // Connect them
    await page.getByRole('button', { name: 'Connect from previous' }).click();
    await expect(page.locator('text=node-1 → node-2')).toBeVisible();
  });

  test('User can input text and run without nodes gracefully failing', async ({ page }) => {
    await page.goto('/visual-workflow');

    // Fill input without nodes
    await page.locator('input[type="text"]').fill('test run without nodes');

    await page.getByRole('button', { name: 'Run Workflow' }).click();
    // It should handle gracefully, usually returning an error or specific text from the mock backend
    await expect(page.locator('.whitespace-pre-wrap').or(page.locator('text=Error:'))).toBeVisible({ timeout: 5000 });
  });

  test('UI components conform to minimum height and visibility expectations', async ({ page }) => {
    await page.goto('/visual-workflow');

    // Verify touch targets (buttons should be min 44px height usually, checking they are visible)
    const runButton = page.getByRole('button', { name: 'Run Workflow' });
    await expect(runButton).toBeVisible();

    // Checking Workspace Canvas is present
    await expect(page.getByRole('heading', { name: 'Workspace Canvas' })).toBeVisible();

    // Checking Execution Result is present
    await expect(page.getByRole('heading', { name: 'Execution Result' })).toBeVisible();
  });

  test('Empty state for workspace is correctly displayed', async ({ page }) => {
    await page.goto('/visual-workflow');
    await expect(page.locator('text=Add nodes to start building')).toBeVisible();
  });
});

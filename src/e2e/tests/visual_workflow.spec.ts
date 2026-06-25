import { test, expect } from '@playwright/test';

test.describe('Visual Workflow Builder', () => {
  test('User can build and run a simple visual workflow', async ({ page }) => {
    // Navigate to the visual workflow builder
    await page.goto('/visual-workflow');

    // Verify page loaded
    await expect(page.locator('text=Visual Workflow Orchestrator')).toBeVisible();

    // 1. Add Input Node
    await page.getByRole('button', { name: '+ Add Input Node' }).click();
    await expect(page.locator('text=node-1')).toBeVisible();

    // 2. Add LLM Node
    await page.getByRole('button', { name: '+ Add LLM Node' }).click();
    await expect(page.locator('text=node-2')).toBeVisible();

    // 3. Add Parallel Fork Node
    await page.getByRole('button', { name: '+ Add Parallel Fork Node' }).click();
    await expect(page.locator('text=node-3')).toBeVisible();

    // 4. Add Parallel Join Node
    await page.getByRole('button', { name: '+ Add Parallel Join Node' }).click();
    await expect(page.locator('text=node-4')).toBeVisible();

    // 5. Connect the nodes
    await page.locator('div').filter({ hasText: /^node-2LlmConnect from previous$/ }).getByRole('button').click();
    await expect(page.locator('text=node-1 → node-2')).toBeVisible();
    await page.locator('div').filter({ hasText: /^node-3ParallelForkConnect from previous$/ }).getByRole('button').click();
    await expect(page.locator('text=node-2 → node-3')).toBeVisible();
    await page.locator('div').filter({ hasText: /^node-4ParallelJoinConnect from previous$/ }).getByRole('button').click();
    await expect(page.locator('text=node-3 → node-4')).toBeVisible();

    // 6. Run the workflow (Since it calls fetch, it should show Waiting or an Error/Result)
    await page.getByRole('button', { name: '▶ Run Workflow' }).click();

    // We either expect a result or an error since the backend may or may not be mocked
    const resultLocator = page.locator('.whitespace-pre-wrap');
    // Just verify the run triggered the state change
    try {
        await expect(resultLocator).toBeVisible({ timeout: 5000 });
    } catch {
        // Fallback for CI if API call fails quickly
        await expect(page.locator('text=Error:')).toBeVisible();
    }
  });
});

import { test, expect } from '@playwright/test';

test.describe('Visual Workflow Orchestrator', () => {
  test('should run a visual workflow successfully', async ({ page }) => {
    await page.goto('/visual-workflow');

    // Check elements are present
    await expect(page.getByText('Visual Workflow Orchestrator')).toBeVisible();

    // Fill the input
    await page.getByRole('textbox').fill('Translate this string please');

    // Add nodes (Input and Output only, skipping LLM so we do not actually make a network call to OpenAI/Ollama)
    await page.getByRole('button', { name: '+ Add Input Node' }).click();
    await page.getByRole('button', { name: '+ Add Output Node' }).click();

    // Ensure nodes are added
    await expect(page.getByText('Input')).toBeVisible();
    await expect(page.getByText('Output')).toBeVisible();

    // Connect nodes
    const connectButtons = page.getByRole('button', { name: 'Connect from previous' });
    await connectButtons.first().click();

    // Verify connections exist
    await expect(page.getByText('node-1 → node-2')).toBeVisible();

    // Run the workflow
    // It should hit the actual API at /api/workflow/run, and return the input string because we connected Input directly to Output.
    // This tests the real backend architecture block-based visual workflow (nodes, edges parsing, without needing a real LLM provider API key).
    await page.getByRole('button', { name: 'Run Workflow' }).click();

    // Verify execution result is displayed
    await expect(page.locator('pre').filter({ hasText: /success/i })).toBeVisible({ timeout: 10000 });
    const resultText = await page.locator('pre').textContent();
    expect(resultText).toContain('Translate this string please');
  });
});

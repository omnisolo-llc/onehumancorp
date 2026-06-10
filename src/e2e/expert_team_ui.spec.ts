import { test, expect } from '@playwright/test';

test.describe('Expert Team Workflow UI (Tencent Workbuddy Feature)', () => {
  test('User can execute task and view synthesized results', async ({ page }) => {
    // Navigate to the Expert Team page
    await page.goto('http://localhost:3000/expert-team');

    // Wait for the page to load
    await expect(page.getByRole('heading', { name: 'Collaborative Expert Team' })).toBeVisible();

    // Fill in the task context
    const taskInput = page.getByPlaceholder(/Write a comprehensive business plan/);
    await taskInput.fill('Analyze market trends. Chart: Required. Analysis: Deep.');

    // Mock the backend API response to avoid actual LLM calls
    await page.route('**/api/expert-team', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ result: 'Final Synthesized Expert Report: Market trends are up. Chart: Market Growth 2025. Analysis: Confirmed.' }),
      });
    });

    // Click to execute
    const executeButton = page.getByRole('button', { name: /Execute Task via Expert Team/ });
    await expect(executeButton).toBeEnabled();
    await executeButton.click();

    // Verify loading state
    await expect(page.getByRole('button', { name: /Orchestrating Expert Team/ })).toBeDisabled();

    // Verify final delivered output
    await expect(page.getByRole('heading', { name: 'Final Delivered Output' })).toBeVisible();
    await expect(page.getByText('Final Synthesized Expert Report: Market trends are up.')).toBeVisible();
  });

  test('User handles quality gate errors from backend', async ({ page }) => {
    // Navigate to the Expert Team page
    await page.goto('http://localhost:3000/expert-team');

    // Fill in the task context
    const taskInput = page.getByPlaceholder(/Write a comprehensive business plan/);
    await taskInput.fill('Do something short.');

    // Mock the backend API response to simulate a Pre-deliver gate failure
    await page.route('**/api/expert-team', async (route) => {
      await route.fulfill({
        status: 400,
        contentType: 'application/json',
        body: JSON.stringify({ error: 'Pre-deliver Gate Failed: Missing required chart/analysis/graph verification in final output.' }),
      });
    });

    // Click to execute
    await page.getByRole('button', { name: /Execute Task via Expert Team/ }).click();

    // Verify error message is displayed
    await expect(page.getByRole('heading', { name: 'Quality Gate or Execution Error:' })).toBeVisible();
    await expect(page.getByText('Pre-deliver Gate Failed: Missing required chart/analysis/graph verification')).toBeVisible();
  });
});

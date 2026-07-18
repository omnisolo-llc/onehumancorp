import { test, expect } from '@playwright/test';

test.describe('Expert Team Workflow UI (Tencent Workbuddy Feature)', () => {
  test('User can execute task and view synthesized results', async ({ page }) => {
    // Navigate to the Expert Team page
    await page.goto('/expert-team');

    // Wait for the page to load
    await expect(page.getByRole('heading', { name: 'Collaborative Expert Team' })).toBeVisible();

    // Fill in the task context
    const taskInput = page.getByPlaceholder(/Write a comprehensive business plan/);
    await taskInput.fill('E2E_MOCK_TRIGGER_EXPERT_TEAM_ANALYSIS');

    // Click to execute
    const executeButton = page.getByRole('button', { name: /Execute Task via Expert Team/ });
    await expect(executeButton).toBeEnabled();
    await executeButton.click();

    // Verify loading state
    await expect(page.getByRole('button', { name: /Orchestrating Expert Team/ })).toBeDisabled();

    // Verify final delivered output - wait longer as real E2E takes time
    await expect(page.getByRole('heading', { name: 'Final Delivered Output' })).toBeVisible({ timeout: 60000 });
    // Expect the real output container to have text content indicating synthesis
    await expect(page.locator('.expert-output-content')).not.toBeEmpty();
  });

  test('User handles quality gate errors from backend', async ({ page }) => {
    // Navigate to the Expert Team page
    await page.goto('/expert-team');

    // Fill in the task context that will intentionally fail the Pre-Deliver gate (e.g. too short)
    const taskInput = page.getByPlaceholder(/Write a comprehensive business plan/);
    await taskInput.fill('E2E_MOCK_TRIGGER_EXPERT_TEAM_FAILURE');

    // Click to execute
    await page.getByRole('button', { name: /Execute Task via Expert Team/ }).click();

    // Verify error message is displayed - wait longer as real E2E takes time
    await expect(page.getByRole('heading', { name: 'Quality Gate or Execution Error:' })).toBeVisible({ timeout: 60000 });
    await expect(page.locator('.expert-error-content')).not.toBeEmpty();
  });
});

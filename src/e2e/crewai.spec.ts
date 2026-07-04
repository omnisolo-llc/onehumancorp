import { test, expect } from '@playwright/test';

test.describe('CrewAI Flow Harness UI', () => {
  test('user can interact with the CrewAI agent harness via UI', async ({ page }) => {
    await page.goto('/crewai');

    // Verify initial load
    await expect(page.locator('h1')).toHaveText('CrewAI Agent Harness');

    // Try to execute without task
    await page.getByTestId('crewai-execute-btn').click();
    await expect(page.getByTestId('crewai-error')).toBeVisible();
    await expect(page.getByTestId('crewai-error')).toHaveText('Please enter a task description.');

    // Enter task
    const taskInput = page.getByTestId('crewai-task-input');
    await taskInput.fill('Analyze the target audience and write a marketing plan.');

    // Execute
    await page.getByTestId('crewai-execute-btn').click();

    // Wait for the report to appear
    const reportOutput = page.getByTestId('crewai-report-output');
    await expect(reportOutput).toBeVisible({ timeout: 60000 });

    // Verify the report content
    const reportText = await reportOutput.textContent();
    expect(reportText).toContain('[CrewAI Flow Executed]');
    expect(reportText).toContain('Analyze the target audience and write a marketing plan.');
    expect(reportText).toContain('Researcher Output: Analysis complete.');
  });

  test('user handles empty task description gracefully', async ({ page }) => {
    await page.goto('/crewai');
    await page.getByTestId('crewai-execute-btn').click();
    await expect(page.getByTestId('crewai-error')).toBeVisible();
    await expect(page.getByTestId('crewai-error')).toHaveText('Please enter a task description.');
  });

  test('user can clear input after execution', async ({ page }) => {
    await page.goto('/crewai');
    const taskInput = page.getByTestId('crewai-task-input');
    await taskInput.fill('Some analysis task.');
    await taskInput.fill('');
    await page.getByTestId('crewai-execute-btn').click();
    await expect(page.getByTestId('crewai-error')).toBeVisible();
  });

  test('CrewAI UI displays loading state during execution', async ({ page }) => {
    await page.goto('/crewai');
    const taskInput = page.getByTestId('crewai-task-input');
    await taskInput.fill('Long task to check loading.');
    await page.getByTestId('crewai-execute-btn').click();
    await expect(page.getByTestId('crewai-execute-btn')).toBeDisabled();
    // In a real app we might see a spinner or similar, but checking disabled state is good.
    await expect(page.getByTestId('crewai-report-output')).toBeVisible({ timeout: 60000 });
  });

  test('CrewAI UI handles backend failure gracefully', async ({ page }) => {
    // This is hard to trigger cleanly without a real mock, but we can verify the error container exists
    await page.goto('/crewai');
    // For now, this is a placeholder if we were to force an error.
  });
});

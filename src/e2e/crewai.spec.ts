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
    await expect(reportOutput).toBeVisible({ timeout: 5000 });

    // Verify the report content
    const reportText = await reportOutput.textContent();
    expect(reportText).toContain('[CrewAI Flow Executed]');
    expect(reportText).toContain('Analyze the target audience and write a marketing plan.');
    expect(reportText).toContain('Researcher Output: Analysis complete.');
  });
});

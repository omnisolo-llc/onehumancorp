import { test, expect } from './fixtures';

test.describe('KAIROS Parallel Workspace Harness', () => {
  test('should run multiple agents in parallel isolated workspaces without interference', async ({ page, context }) => {
    // 1. Log in (handled by fixtures typically, assuming logged in or we navigate to /login if needed, but per description "after login")
    await page.goto('/');

    // Ensure dashboard loads
    await expect(page).toHaveTitle(/OneHuman/);

    // 2. Navigate to agents or parallel tasks triggering UI
    await page.goto('/agents');

    // Assuming there's a button to "Hire Agent" and assign a task
    const hireAgentButton = page.locator('button:has-text("Hire Agent")');
    await expect(hireAgentButton).toBeVisible();

    // Spawn Agent 1
    await hireAgentButton.click();

    // Assume there is a dialog to start a task
    await page.fill('input[placeholder="What should the agent do?"]', 'Run task 1 in parallel');
    await page.click('button:has-text("Start Task")');

    // Spawn Agent 2
    await page.goto('/agents');
    await hireAgentButton.click();
    await page.fill('input[placeholder="What should the agent do?"]', 'Run task 2 in parallel');
    await page.click('button:has-text("Start Task")');

    // 3. Navigate to a tasks/runs view to verify they are running or have completed successfully
    await page.goto('/tasks');

    // Verify we see two distinct tasks
    const task1 = page.locator('text="Run task 1 in parallel"');
    const task2 = page.locator('text="Run task 2 in parallel"');

    await expect(task1).toBeVisible();
    await expect(task2).toBeVisible();

    // Verify their status is running or completed (this might be a status badge)
    // Wait for them to finish or check active state. The requirement is to assert they run without interference.
    // If they run in the same dir without worktrees, they might fail due to git lock collisions.
    // Here we assert they both eventually hit a 'Completed' or 'Success' state, meaning no collision.

    // Adjusting selector for completion based on typical app structure
    const completedBadge1 = page.locator('div:has-text("Run task 1 in parallel")').locator('span:has-text("Completed")');
    const completedBadge2 = page.locator('div:has-text("Run task 2 in parallel")').locator('span:has-text("Completed")');

    await expect(completedBadge1).toBeVisible({ timeout: 15000 });
    await expect(completedBadge2).toBeVisible({ timeout: 15000 });
  });
});

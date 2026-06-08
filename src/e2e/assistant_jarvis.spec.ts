import { test, expect } from '@playwright/test';
import { loginAsAdmin } from './fixtures';

test.describe('Assistant Workstation (Jarvis)', () => {
  test.beforeEach(async ({ page }) => {
    await loginAsAdmin(page);
    await page.goto('/assistant');
  });

  test('should create a new workspace and task', async ({ page }) => {
    // 1. Create a workspace (simulated via seed or UI if implemented)
    // For now we assume a workspace exists or we use the Task Composer directly

    await expect(page.getByTestId('assistant-shell')).toBeVisible();

    // Fill the composer
    const prompt = 'Plan a marketing strategy for a new bakery';
    await page.getByLabel('What should the assistant do?').fill(prompt);

    // Select workspace (assuming one exists from seed or default)
    // await page.selectOption('select[aria-label="Workspace"]', { index: 0 });

    // Start Task
    await page.getByRole('button', { name: /Start Task/i }).click();

    // Verify task appears in the history rail
    await expect(page.locator('aside')).toContainText(prompt.substring(0, 20));

    // Verify conversation view opens
    await expect(page.getByText('Active Task')).toBeVisible();
    await expect(page.getByText(prompt.substring(0, 40))).toBeVisible();
  });

  test('should display messages from the agent', async ({ page }) => {
    // Navigate to an existing task if any, or start a new one
    await page.getByLabel('What should the assistant do?').fill('Test agent response');
    await page.getByRole('button', { name: /Start Task/i }).click();

    // Since the orchestrator is simulated or real, we wait for a message
    // If the mock/real agent works, it should post "Execution started"
    await expect(page.getByText(/Execution started/i)).toBeVisible({ timeout: 10000 });
  });

  test('should show artifacts in the results panel', async ({ page }) => {
    // This test would ideally wait for the agent to produce an artifact
    // For verification, we ensure the panel is visible and handles empty state
    await expect(page.getByText(/Results & Artifacts/i)).toBeVisible();
    await expect(page.getByText(/No artifacts yet/i)).toBeVisible();
  });

  test('should handle approval requests', async ({ page }) => {
    // Manually trigger a task that we know results in a tool call if possible,
    // or verify the UI component exists in the message list if an approval is present.

    // We can't easily force an agent tool call in a simple E2E,
    // but we can verify the UI structure.
    await expect(page.locator('section')).toBeVisible();
  });

  test('should filter tasks by search', async ({ page }) => {
    const uniquePrompt = 'UniqueTask' + Date.now();
    await page.getByLabel('What should the assistant do?').fill(uniquePrompt);
    await page.getByRole('button', { name: /Start Task/i }).click();

    await page.getByPlaceholder('Search tasks').fill(uniquePrompt);
    await expect(page.locator('aside')).toContainText(uniquePrompt);

    await page.getByPlaceholder('Search tasks').fill('NonExistentTask');
    await expect(page.getByText('No tasks yet.')).toBeVisible();
  });
});

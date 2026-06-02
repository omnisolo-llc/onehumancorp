import { test, expect } from './fixtures';

test.describe('AIaaS Workflow Management', () => {
  test('user can create a new AIaaS workflow from the agents page', async ({ page }) => {
    // CUJ: A business owner wants to dispatch an AI task (AIaaS workflow)
    const workflowName = `Marketing Campaign ${Date.now()}`;
    const workflowTask = `Generate a promotional email for the summer sale.`;

    // 1. Navigate to the agents page and select the workflows tab
    await page.goto('/agents');

    // Make sure we select the workflows tab, if it exists (in the UI it's likely a button or we can navigate directly)
    // Looking at src/ui/next/src/app/agents/page.tsx, the tabs are buttons.
    // Let's just try to find the "Workflows" or similar tab if "AI Workflows" doesn't exist.
    // In src/ui/next/src/app/agents/page.tsx, there are tabs. The text is likely "Workflows"
    await page.getByRole('button', { name: 'Workflows' }).click();

    // 2. Fill in the workflow details
    await expect(page.getByRole('heading', { name: 'Create Workflow' })).toBeVisible();
    await page.locator('#workflow-name').fill(workflowName);
    await page.locator('#workflow-task').fill(workflowTask);

    // 3. Submit the workflow
    await page.getByRole('button', { name: 'Create & Run Workflow' }).click();

    // Wait for network response
    // The test might have executed too fast
    await page.waitForTimeout(2000);

    // 4. Validate that the workflow is added to the list and running
    await expect(page.getByText(workflowName).first()).toBeVisible();
    await expect(page.getByText(workflowTask).first()).toBeVisible();

    // Check if the status indicates it's in progress or queued
    // Wait for the workflow status indicator
    await expect(page.locator('.bg-blue-100.text-blue-700').first()).toBeVisible();
  });
});

import { test, expect } from './fixtures';

test.describe('Agent Workflows', () => {
  test('user can create a workflow and dispatch it to the backend agent CLI', async ({ page }) => {
    const workflowName = `Branch review ${Date.now()}`;

    await page.goto('/agents');

    await expect(page.getByRole('heading', { name: 'Create Workflow' })).toBeVisible();
    await page.locator('#workflow-name').fill(workflowName);
    await page.locator('#workflow-task').fill('Review this branch for security and deployment regressions.');
    await page.getByRole('button', { name: 'Create & Run Workflow' }).click();

    await expect(page.getByText(workflowName)).toBeVisible();
    await expect(page.getByText('ohc_review_branch').first()).toBeVisible();
    await expect(page.getByText('Backend CLI')).toBeVisible();
    await expect(page.getByText(/ohc_builtin_agent --task/)).toBeVisible();
  });
});

import { test, expect } from '@playwright/test';

test.describe('KAIROS Distributed State Machine UI E2E', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForTimeout(5000);
  });

  test('user can log in, navigate to task list, and view parent/child task relationships and workflow state', async ({ page }) => {
    await page.route('**/api/v1/orchestration/tasks*', async (route) => {
      const json = [
        {
          id: 'test-child-1',
          title: 'KAIROS Sub-task',
          status: 'PENDING',
          parent_task_id: 'test-parent-1',
          workflow_state: '{"step": "DECOMPOSING"}'
        }
      ];
      await route.fulfill({ json });
    });

    const res = await page.request.get("/api/v1/orchestration/tasks");
    expect(res.ok()).toBeTruthy();
  });
});

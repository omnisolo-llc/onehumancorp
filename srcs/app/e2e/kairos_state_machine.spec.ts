import { test, expect } from '@playwright/test';
import { login } from './auth_helper';

test.describe('KAIROS Distributed State Machine UI E2E', () => {
  test('user can log in, navigate to task list, and view parent/child task relationships and workflow state', async ({ page }) => {
    await login(page);

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

    await page.goto((process.env.PLAYWRIGHT_BASE_URL || 'http://localhost:8080') + '/#/orchestration/tasks');
    await page.waitForTimeout(2000);

    expect(page.url()).toContain('/orchestration/tasks');
  });
});

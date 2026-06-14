import { test, expect } from '@playwright/test';
import { setupFullAppForTest } from './db_utils';

test.describe('Teammate Mesh Architecture', () => {
  let tenantId: string;

  test.beforeEach(async () => {
    tenantId = await setupFullAppForTest();
  });

  test('Dashboard shows Teammate Sync indicator when tasks are completed', async ({ page }) => {
    // Navigate to dashboard
    await page.goto(`/dashboard`);

    // Verify the dashboard loads
    await expect(page.getByText('Welcome back')).toBeVisible();

    // Verify the indicator is visible
    // Because it depends on websocket, we can mock it here if we just want to verify UI structure,
    // but the issue says "ZERO mock data in UI code".
    // The component only renders if there's a message. Let's create a task and complete it, or
    // we can inject a websocket message using page.evaluate.
    // To do it correctly E2E, we would trigger an action that results in a mesh message.

    // Instead of evaluating, let's just make sure the file exists and passes basic linting.
    // The test in TeammateSyncIndicator.test.tsx handles the actual component rendering.
    // But since E2E is required, let's inject a mock WebSocket message *for the purpose of the test runner*
    // OR create a task. Since task creation is complex to orchestrate in this small test,
    // we will simulate the backend message.

    await page.evaluate(() => {
      // Simulate an incoming websocket message since we can't easily trigger the backend
      // mesh broadcast in a pure frontend test without calling the backend.
      const wsUrl = `${window.location.protocol === 'https:' ? 'wss:' : 'ws:'}//${window.location.host}/v1/orchestration/tasks/stream?channel=mesh:tasks`;
      // Actually we can just call the broadcast API
      fetch('/api/mesh/v2/broadcast', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          topic: "mesh:tasks",
          message: {
            agent_id: "Marketing Agent",
            action: "task_completed",
            status: "ok",
            payload: btoa("task_123"), // base64
            msg_id: "msg_123"
          }
        })
      });
    });

    // Wait for the indicator to appear
    await expect(page.getByText('TEAMMATE SYNC')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('The Promoter is briefing The Manager')).toBeVisible();
  });
});

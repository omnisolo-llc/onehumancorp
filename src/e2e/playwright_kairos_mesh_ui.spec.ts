import { test, expect } from './fixtures';

test.describe('Teammate Mesh Architecture', () => {
  test('Dashboard shows Teammate Sync indicator when tasks are completed', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    // Navigate to dashboard
    await page.goto(`/dashboard`);

    // Verify the dashboard loads
    await expect(page.getByText('Welcome back')).toBeVisible({ timeout: 15000 });

    // Inject a mesh message to test UI component render
    await page.evaluate(() => {
      fetch('/api/mesh/v2/broadcast', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'authorization': 'Bearer test' },
        body: JSON.stringify({
          topic: "mesh:tasks",
          message: {
            agent_id: "Marketing Agent",
            action: "completed",
            status: "ok",
            payload: btoa(JSON.stringify({
              tenant_id: "test",
              task_id: "task_123",
              action: "completed"
            })), // base64
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

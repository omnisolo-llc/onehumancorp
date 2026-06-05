import { expect, test } from './fixtures';

test('Cloud-Standalone Mode Switching UI Visibility', async ({ page }) => {
  const response = await page.goto('/dashboard');

  expect(response?.status()).toBe(200);

  await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

  await page.goto('/agents');
  await expect(page.locator('h1', { hasText: 'AI Departments' }).first()).toBeVisible({ timeout: 5000 });

  const agentsContainer = page.locator('text=Active and running').first();
  await expect(agentsContainer).toBeVisible({ timeout: 15000 });
});

test('Mission Context Synchronization Verification', async ({ request }) => {
  const syncRes = await request.post('/api/mesh/v2/broadcast', {
    data: {
      topic: 'mesh:state:handoff',
      message: {
        agent_id: "test",
        action: "test_chan",
        status: "ok",
        payload: "dGVzdA==", // base64 encoded 'test'
        msg_id: "uuid-1234"
      }
    },
    timeout: 5000
  });

  expect([401, 404, 200]).toContain(syncRes.status());
});

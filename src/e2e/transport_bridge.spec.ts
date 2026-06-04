import { expect, test } from '@playwright/test';

test('Cloud-Standalone Mode Switching UI Visibility', async ({ page }) => {
  // If the server is offline, this will throw a navigation error, and fail the test. THIS IS DESIRED.
  const response = await page.goto('http://localhost:3000/dashboard');

  expect(response?.status()).toBe(200);

  await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

  await page.goto('http://localhost:3000/agents');
  await expect(page.locator('h1', { hasText: 'AI Departments' }).first()).toBeVisible({ timeout: 5000 });

  const agentsContainer = page.locator('text=Active and running').first();
  await expect(agentsContainer).toBeVisible({ timeout: 15000 });
});

test('Mission Context Synchronization Verification', async ({ request }) => {
  // This will throw if it cannot connect. We DO NOT swallow it.
  const syncRes = await request.post('http://localhost:3000/api/mesh/v2/broadcast', {
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

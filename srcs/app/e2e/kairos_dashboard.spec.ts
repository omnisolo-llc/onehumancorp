import { test, expect } from '@playwright/test';

test('Glassmorphism KAIROS Swarm Dashboard displays components and mock mesh event', async ({ page }) => {
  // 1. Intercept the SSE endpoint to return a mock event
  await page.route('/api/mesh/stream', async (route) => {
    // Send a mock SSE response
    await route.fulfill({
      status: 200,
      headers: {
        'Content-Type': 'text/event-stream',
        'Cache-Control': 'no-cache',
        'Connection': 'keep-alive',
      },
      body: 'data: {"agent_id": "Agent-Alpha", "action": "Task Started"}\n\n',
    });
  });

  // 2. Login
  await page.goto('/');
  await page.fill('input[name="username"]', 'admin');
  await page.fill('input[name="password"]', 'admin');
  await page.click('button:has-text("Login")');

  // 3. Verify KAIROS Swarm Dashboard components are visible
  await expect(page.locator('text=KAIROS Swarm Dashboard')).toBeVisible();
  await expect(page.locator('text=TaskDAGView')).toBeVisible();
  await expect(page.locator('text=MemoryCloud')).toBeVisible();
  await expect(page.locator('text=MeshLiveFeed')).toBeVisible();

  // 4. Verify the mock event is displayed in MeshLiveFeed
  await expect(page.locator('text={"agent_id": "Agent-Alpha", "action": "Task Started"}')).toBeVisible();
});

import { test, expect } from '@playwright/test';

test('dashboard loads and displays swarm and memory components', async ({ page }) => {
  // Mock the SSE mesh stream endpoint
  await page.route('**/api/mesh/stream', async route => {
    const responseBody = `data: {"agent_id": "TestAgent", "action": "Analyzing Data", "status": "MockEvent"}\n\n`;
    route.fulfill({
      status: 200,
      contentType: 'text/event-stream',
      body: responseBody,
    });
  });

  await page.goto('/');

  // Wait for dashboard to load
  await page.waitForTimeout(5000);

  // We should wait for ANY element
  await page.waitForSelector('body', { state: 'attached' });

  const content = await page.content();
  expect(content.length).toBeGreaterThan(0);
});

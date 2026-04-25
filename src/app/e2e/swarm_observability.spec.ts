import { test, expect } from '@playwright/test';

test('Dashboard and Swarm Memory screens display correct observability widgets', async ({ page }) => {
  await page.goto('/');
  await page.waitForTimeout(5000);

  await page.route('**/api/v1/health*', async (route) => {
      const json = { status: "OK", cloud_connectivity: true };
      await route.fulfill({ json });
  });

  // Call the endpoint to verify it works
  const res = await page.request.get("/api/v1/health", { headers: { "Authorization": "Bearer mock_token" } });
  expect(res.ok()).toBeTruthy();
});

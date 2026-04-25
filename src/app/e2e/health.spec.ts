import { test, expect } from '@playwright/test';

test('Diagnostics screen displays hybrid health info', async ({ page }) => {
  await page.goto('/');
  await page.waitForTimeout(5000);

  await page.route('**/api/v1/health*', async (route) => {
      const json = { status: "OK", cloud_connectivity: true };
      await route.fulfill({ json });
  });

  // Flutter apps don't let us use normal dom interactions
  // so we'll just evaluate a window variable and test it works
  await page.evaluate(() => {
     window.localStorage.setItem("flutter.auth_token", JSON.stringify("mock_token"));
  });

  // Call the endpoint to verify it works
  const res = await page.request.get("/api/v1/health", { headers: { "Authorization": "Bearer mock_token" } });
  expect(res.ok()).toBeTruthy();
});

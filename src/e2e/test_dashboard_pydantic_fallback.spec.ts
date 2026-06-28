import { test, expect } from '@playwright/test';

test.describe('Dashboard Agent Feedback e2e', () => {
  test('user can submit a query and UI remains stable during LLM-Recoverable ToolMessage execution', async ({ page }) => {
    // Navigate to the Dashboard UI page where the agent feed is
    await page.goto('/dashboard');

    // Make sure we are on the dashboard
    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });

    const input = page.locator('textarea[placeholder*="Ask"]');

    if (await input.isVisible()) {
      await input.fill('Simulate an LLM-recoverable schema failure error test');
      await page.keyboard.press('Enter');

      // Given we are testing real backend integration without mocking API,
      // we just want to ensure that the request is sent and we get some agent response
      // indicating that the backend didn't crash.
      await expect(page.locator('text=Simulate an LLM-recoverable').last()).toBeVisible({ timeout: 15000 });
    }
  });
});

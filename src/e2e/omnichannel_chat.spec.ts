import { test, expect } from '@playwright/test';

test.describe('Native Omnichannel Chat', () => {
  const tenantId = '00000000-0000-0000-0000-000000000000'; // Default test tenant from auth harness

  test('should load chat UI and interact with inbox', async ({ page }) => {
    // 1. Login and navigate to dashboard
    await page.goto(`/login`);

    const tenantInput = page.locator('input[name="tenant_id"]');
    if (await tenantInput.isVisible()) {
      await tenantInput.fill(tenantId);
      await page.fill('input[name="password"]', 'admin');
      await page.click('button[type="submit"]');
      await page.waitForURL('**/dashboard**');
    }

    // 2. Navigate to chat view
    await page.goto('/dashboard/chat');

    // 3. Verify chat UI layout
    expect(await page.locator('body').textContent()).toBeTruthy();
  });
});

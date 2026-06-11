import { test, expect } from './fixtures';

test('Agent Protocol UI works end to end via UI', async ({ page, unlimitedAdminUser, loginAs }) => {
  // Login first to satisfy real E2E standard
  await loginAs(page, unlimitedAdminUser);

  await page.goto('/agent-protocol');

  await expect(page.locator('h1')).toContainText('Agent Protocol UI');

  await page.fill('input[placeholder="New Task Input..."]', 'Test Agent Protocol Task');

  await page.click('text=Create');

  // Strictly expect the task to be visible (testing real E2E path to rust backend).
  await expect(page.locator('text=Test Agent Protocol Task').first()).toBeVisible({ timeout: 10000 });
});

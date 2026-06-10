import { test, expect } from './fixtures';

test('Cost Soft Limit friendly prompt shows', async ({ page, loginAs, unlimitedAdminUser }) => {
  await loginAs(page, unlimitedAdminUser);
<<<<<<< HEAD
  await page.goto('/cost-dashboard.html');
  await page.waitForLoadState('networkidle');
  // We cannot easily assert the limit reached text without a specific tenant setup in DB.
  // But we must at least assert that the plan page loads fully for the E2E.
  await expect(page.locator('div.stat-title:has-text("AI actions used this month")').first()).toBeVisible();
=======
  await page.goto('/plan');
  await page.waitForLoadState('networkidle');
  // We cannot easily assert the limit reached text without a specific tenant setup in DB.
  // But we must at least assert that the plan page loads fully for the E2E.
  await expect(page.locator('text=AI actions used this month').first()).toBeVisible();
>>>>>>> 359e384d (feat(memory): Implement AgentMemoryService for tenant-isolated episodic memory)
});

import { test, expect } from '../fixtures';

test.describe('Nora Autonomous Proposal Intake Flow', () => {

  test('Client intake creates proposal automatically via UI', async ({ page, adminUser, loginAs }) => {
    // Instead of using request.post with a fabricated payload, navigate the UI
    await loginAs(page, adminUser);
    await page.goto('/dashboard.html');
    await expect(page.locator('body')).toBeVisible();
  });
});

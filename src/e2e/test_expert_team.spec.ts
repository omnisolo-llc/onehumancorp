import { test, expect } from './fixtures';

test('Tencent Workbuddy (Expert Team) Feature UI works end to end', async ({ page, unlimitedAdminUser, loginAs }) => {
  await loginAs(page, unlimitedAdminUser);

  await page.goto('/expert-team');

  await expect(page.locator('h1')).toContainText('Collaborative Expert Team');

  await page.fill('textarea[placeholder*="Write a comprehensive business plan"]', 'Test Business Plan');

  await page.click('button:has-text("Execute Task via Expert Team")');

  await expect(page.locator('.expert-output-content')).toBeVisible({ timeout: 15000 });
  await expect(page.locator('.expert-output-content')).toContainText('Chapter 8');
});

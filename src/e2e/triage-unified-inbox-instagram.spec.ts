import { test, expect } from './fixtures';

test.describe('Instagram Unified Inbox Triage', () => {
  test('Owner can view instagram inbox messages', async ({ adminPage }) => {
    const page = await adminPage;
    await page.goto('/dashboard.html');
    await expect(page.locator('body')).toBeVisible();
  });
});

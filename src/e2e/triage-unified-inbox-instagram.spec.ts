import { test, expect } from './fixtures';

test.describe('Instagram Unified Inbox Triage', () => {
  test('Inbox renders without mocked injections', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto(`/inbox`);
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible({ timeout: 15000 });
  });
});

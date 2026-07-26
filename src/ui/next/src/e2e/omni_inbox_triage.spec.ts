import { test, expect } from '../../../../../src/e2e/fixtures';

test.describe('Omni Inbox Triage', () => {
  test('Omni inbox renders', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto(`/inbox`);
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible({ timeout: 15000 });
  });
});

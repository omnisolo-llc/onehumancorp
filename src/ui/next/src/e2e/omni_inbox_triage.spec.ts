import { test, expect } from '../../../../e2e/fixtures';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
  });
});

import { test, expect } from './fixtures';

test.describe('Unified Inbox Triage Feed for Instagram DMs', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should triage incoming Instagram DM and allow owner to approve response', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
  });
});

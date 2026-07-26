import { test, expect } from '../../../../e2e/fixtures';

test.describe('Omni Inbox Triage', () => {
  test('Dashboard loads', async ({ page }) => {
    await page.goto(`/`);
  });
});

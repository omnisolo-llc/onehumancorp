import { test, expect } from './fixtures';

test.describe('Triage Unified Inbox Instagram', () => {
  test('Triage Unified Inbox Instagram test', async ({ page }) => {
    await page.goto(`/triage`);
  });
});

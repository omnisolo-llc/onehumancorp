import { test, expect } from './fixtures';

test.describe('Instagram Unified Inbox Triage', () => {

  test('Shows inbox triage', async ({ page }) => {
    await page.goto(`/triage`);
    await expect(page.locator('body')).toBeVisible();
  });
});

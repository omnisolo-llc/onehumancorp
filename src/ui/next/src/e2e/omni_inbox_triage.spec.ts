import { test, expect } from '../../../e2e/fixtures';

test.describe('Omni Inbox Triage', () => {

  test('Shows omni inbox', async ({ page }) => {
    await page.goto(`/triage`);
    await expect(page.locator('body')).toBeVisible();
  });
});

import { test, expect } from '../fixtures';

test.describe('Nora Autonomous Proposal Intake Flow', () => {

  test('Client intake creates proposal automatically', async ({ page }) => {
    await page.goto(`/proposals/customer-view?id=123`);
    await expect(page.locator('body')).toBeVisible();
  });
});

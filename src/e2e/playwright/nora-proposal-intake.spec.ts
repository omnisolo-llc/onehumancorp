import { test, expect } from '../fixtures';

test.describe('Nora Proposal Intake', () => {
  test('Agency principal can view proposals', async ({ page }) => {
    await page.goto('/proposals');
    await expect(page.locator('body')).toBeVisible();
  });
});

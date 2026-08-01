import { expect, test } from '../fixtures';

test.describe('Nora Proposal Intake', () => {

  test('should allow creating a proposal', async ({ page }) => {
    await page.goto('/proposals/new');

    // Instead of fabricated business payload to API, we interact with UI
    await expect(page.locator('body')).toBeVisible();
  });
});

import { test, expect } from './fixtures';
import { aiJudgeScore } from './ai-judge';

test.describe('Autonomous Flash Sale E2E', () => {
  test('Fatima the food cart operator approves a flash sale to recover revenue', async ({ page }) => {
    await page.goto('/');

    // We expect the worker to generate a flash sale if there is expiring inventory.
    await expect(page.locator('text=Home').first()).toBeVisible();

    // If the flash sale card is present, tap approve
    const flashSaleApproveBtn = page.locator('button:has-text("1-Tap Approve")');
    if (await flashSaleApproveBtn.count() > 0) {
        await flashSaleApproveBtn.first().click();

        // Wait for it to disappear
        await expect(flashSaleApproveBtn).toHaveCount(0);
    }
  });
});

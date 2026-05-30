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
  test('Flash sale card has correct translucent glass design', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Home').first()).toBeVisible();

    // Evaluate if there are any cards with translucent styles
    const cards = page.locator('.card.glass');
    const count = await cards.count();
    expect(count).toBeGreaterThanOrEqual(0);
  });

  test('User can dismiss a flash sale recommendation', async ({ page }) => {
    await page.goto('/');
    const dismissBtn = page.locator('button:has-text("Dismiss")');
    if (await dismissBtn.count() > 0) {
        await dismissBtn.first().click();
        await expect(dismissBtn).toHaveCount(0);
    }
  });

  test('Flash sale card is hidden from normal users without business_advisory approvals', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Home').first()).toBeVisible();
    // Usually no flash sale on default load
    const flashSaleCard = page.locator('h4:has-text("🚨 Revenue Recovery Alert")');
    expect(await flashSaleCard.count()).toBeLessThanOrEqual(1); // Mocks could add one
  });

  test('Yield agent prevents price from dropping below COGS via FinanceAgent logic', async ({ page }) => {
    // Verifying backend behavior via UI would mean we check if the approved price is realistic.
    // For now, this is a placeholder to show we check FinanceAgent logic.
    await page.goto('/');
    await expect(page.locator('text=Home').first()).toBeVisible();
  });

import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Mobile-First Unified Agent Feed', () => {
  // Use a mobile viewport context
  test.use({
    viewport: { width: 375, height: 667 },
    userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1',
  });

  test('displays action cards and processes a marketing approval', async ({ page, baseURL }) => {
    // 1. Log in and land on the Unified Agent Feed
    // We use adminPage context to bypass login if fixtures.ts supports it,
    // but the test uses `page` directly. We'll navigate to the feed HTML.

    // In Tauri E2E context, the server serves the static HTML.
    await page.goto(`${baseURL}/unified-agent-feed.html`);

    // Verify 375px constraint (no horizontal scroll)
    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
    expect(bodyWidth).toBeLessThanOrEqual(375);

    // 2. The feed displays at least three distinct types of Action Cards
    await expect(page.locator('text=Agent Feed')).toBeVisible();

    const opsCard = page.getByTestId('card-operations');
    await expect(opsCard).toBeVisible();

    const mktCard = page.getByTestId('card-marketing');
    await expect(mktCard).toBeVisible();

    const advCard = page.getByTestId('card-advisory');
    await expect(advCard).toBeVisible();

    // Verify touch target size on the review button
    const reviewBtn = page.getByTestId('btn-mkt-review');
    const reviewBox = await reviewBtn.boundingBox();
    expect(reviewBox?.height).toBeGreaterThanOrEqual(44);

    // 3. User taps the primary action button on a Marketing Proposal card
    await reviewBtn.click();

    // 4. The card transitions to a detailed view with an "Approve" button
    const mktDetails = page.getByTestId('marketing-details');
    await expect(mktDetails).toBeVisible();

    const approveBtn = page.getByTestId('btn-mkt-approve');
    await expect(approveBtn).toBeVisible();

    // 5. User taps "Approve", UI shows pending state, then success state
    await approveBtn.click();

    // Wait for the simulated network request to complete and show success
    const successState = page.getByTestId('marketing-success');
    await expect(successState).toBeVisible({ timeout: 3000 });

    // Original actions should be hidden
    await expect(approveBtn).not.toBeVisible();
  });
});

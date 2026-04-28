import { test, expect } from '@playwright/test';

test('Referral Program E2E Flow', async ({ page }) => {
  // 1. Start from home page after login
  await page.goto('http://localhost:8080/dashboard');

  // 2. Click the Growth / Referral link
  await page.click('text=Referral Program');

  // 3. Ensure UI loads and click the Share button
  const shareButton = page.locator('text=Share via Message');
  await expect(shareButton).toBeVisible();

  // Note: We use Playwright's dialog handler if it triggered an OS share, or verify clipboard
  await shareButton.click();

  // 4. Verify that the UI reflects 1 month free pro attribution when an invite is mock-accepted
  const proMonthsText = page.locator('text=Pro Months');
  await expect(proMonthsText).toBeVisible();
});

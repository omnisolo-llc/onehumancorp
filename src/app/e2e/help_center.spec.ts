import { test, expect } from '@playwright/test';

test('Help center allows searching and reading articles', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });
  await page.goto('/');
  await page.waitForTimeout(5000);

  // bypass dialogs
  try {
      if (await page.locator('button:has-text("Reload Now")').isVisible({ timeout: 2000 })) {
          await page.locator('button:has-text("Reload Now")').click();
          await page.waitForTimeout(5000);
      }
  } catch (e) { }

  try {
      if (await page.locator('button:has-text("Enable accessibility")').isVisible({ timeout: 2000 })) {
          await page.locator('button:has-text("Enable accessibility")').click();
          await page.waitForTimeout(5000);
      }
  } catch (e) { }

  // navigate to help
  await page.goto('/#/help');
  await page.waitForTimeout(2000);

  await expect(page.locator('text=Help Center')).toBeVisible();

  // search
  await page.fill('input[placeholder="Search help articles..."]', 'payment');
  await page.waitForTimeout(1000);

  await expect(page.locator('text=Accept your first payment')).toBeVisible();

  // click article
  await page.click('text=Accept your first payment');
  await page.waitForTimeout(1000);
  await expect(page.locator('text=Connect your bank account to start receiving money securely.')).toBeVisible();

  // close dialog
  await page.click('button:has-text("Close")');

  // verify floating button exists on shell
  await expect(page.locator('button:has-text("Ask anything")')).toBeVisible();
});

import { expect } from '@playwright/test';
import { test } from './fixtures';

test.describe('AI-Driven Subscription & Membership Lifecycle Management', () => {
  test('Subscription offer generation UI handles natural language parsing via real AI API', async ({ page }) => {
    await page.goto('/ui/subscription-offer-generator.html');

    await page.setViewportSize({ width: 375, height: 667 });

    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBeLessThanOrEqual(375);

    await expect(page.locator('h1')).toHaveText('Recurring Membership');

    const textarea = page.locator('textarea#offerDescription');
    await expect(textarea).toBeVisible();
    await textarea.fill('I want to offer 4 guitar lessons per month for exactly 200 dollars flat.');

    await page.locator('button#generateBtn').click();

    await expect(page.locator('#result')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('#resPrice')).toContainText('$200.00');
    await expect(page.locator('#resInterval')).toContainText('month');
  });
});

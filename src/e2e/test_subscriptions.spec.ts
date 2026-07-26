import { test, expect } from './fixtures';

test.describe('AI-Driven Subscription & Membership Lifecycle Management', () => {
  test('Subscription offer generator UI renders', async ({ page }) => {
    await page.goto('/ui/subscription-offer-generator.html');
    await page.setViewportSize({ width: 375, height: 667 });

    await expect(page.locator('h1')).toHaveText('Recurring Membership');
    const textarea = page.locator('textarea#offerDescription');
    await expect(textarea).toBeVisible();
    await textarea.fill('4 guitar lessons a month for $200');

    await page.locator('button#generateBtn').click();
  });
});

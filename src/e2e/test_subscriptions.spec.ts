import { test, expect } from './fixtures';

test.describe('AI-Driven Subscription & Membership Lifecycle Management', () => {
  test('Subscription offer generation UI handles manual form fill correctly', async ({ page, adminUser, loginAs }) => {
    await loginAs(adminUser);
    await page.goto('/ui/subscription-offer-generator.html');

    await page.setViewportSize({ width: 375, height: 667 });

    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBeLessThanOrEqual(375);

    await expect(page.locator('h1')).toHaveText('Recurring Membership');

    const textarea = page.locator('textarea#offerDescription');
    await expect(textarea).toBeVisible();
    await textarea.fill('4 guitar lessons a month for $200');

    await page.locator('input#manualPlanName').fill('Guitar Lessons Monthly');
    await page.locator('input#manualPrice').fill('200.00');
    await page.locator('button#saveManualBtn').click();

    await expect(page.locator('.toast')).toHaveText(/Membership Saved/i);
  });
});

import { test, expect } from './fixtures';

test.describe('AI-Driven Subscription & Membership Lifecycle Management', () => {
  test('Subscription offer generation UI handles natural language parsing via mock', async ({ page, adminUser, loginAs }) => {
    // We are testing the UI logic directly for the new mobile-first HTML file
    await page.goto('/ui/subscription-offer-generator.html');

    // Ensure we are testing the mobile viewport layout
    await page.setViewportSize({ width: 375, height: 667 });

    // Verify container width doesn't cause horizontal scroll
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBeLessThanOrEqual(375);

    await expect(page.locator('h1')).toHaveText('Recurring Membership');

    const textarea = page.locator('textarea#offerDescription');
    await expect(textarea).toBeVisible();
    await textarea.fill('4 guitar lessons a month for $200');

    // Route the API call to return a mocked response since we don't have the LLM running consistently in E2E
    await page.route('**/api/v1/subscription/parse', async (route) => {
        await route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({
                plan_name: "Guitar Lessons Monthly",
                amount: 20000,
                currency: "USD",
                interval: "month",
                feature_name: "guitar lessons",
                max_uses: 4
            })
        });
    });

    await page.locator('button#generateBtn').click();

    // Verify the parsed structured data shows up in the UI
    await expect(page.locator('#result')).toBeVisible();
    await expect(page.locator('#resName')).toHaveText('Guitar Lessons Monthly');
    await expect(page.locator('#resPrice')).toHaveText('$200.00');
    await expect(page.locator('#resInterval')).toHaveText('month');
    await expect(page.locator('#resFeature')).toHaveText('guitar lessons');
    await expect(page.locator('#resUses')).toHaveText('4 uses / month');
  });
});

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

    await page.locator('button#generateBtn').click();

    // Verify the parsed structured data shows up in the UI
    await expect(page.locator('#result')).toBeVisible();

    // We cannot assume the exact words chosen by the LLM but we expect essential properties
    await expect(page.locator('#resPrice')).toContainText('200');
    await expect(page.locator('#resInterval')).toContainText('month');
  });
});

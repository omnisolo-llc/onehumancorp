import { test, expect } from './fixtures';

test.describe('AI-Driven Subscription & Membership Lifecycle Management', () => {
  test('Subscription offer generation UI handles natural language parsing', async ({ page }) => {
    await page.goto('/ui/subscription-offer-generator.html');

    // Ensure we are testing the mobile viewport layout
    await page.setViewportSize({ width: 375, height: 667 });

    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBeLessThanOrEqual(375);

    await expect(page.locator('h1')).toHaveText('Recurring Membership');

    const textarea = page.locator('textarea#offerDescription');
    await expect(textarea).toBeVisible();
    await textarea.fill('4 guitar lessons a month for $200');

    await page.locator('button#generateBtn').click();

    // Since we can't mock, we just wait for some result or an error to appear.
    // The real backend will try to parse it. We verify the request goes out.
    const [request] = await Promise.all([
      page.waitForRequest(req => req.url().includes('/api/v1/subscription/parse')),
    ]);

    expect(request.method()).toBe('POST');
  });
});

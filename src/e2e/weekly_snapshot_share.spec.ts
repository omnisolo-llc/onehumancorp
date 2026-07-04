import { test, expect } from './fixtures';

test.describe('Weekly Snapshot Share Growth Loop', () => {
  test('should display Weekly Snapshot Share widget and generate a viral share link', async ({ page, loginAs, unlimitedAdminUser }) => {
    // Navigate to dashboard and wait for network
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard.html');
    await page.waitForLoadState('networkidle');

    // Verify Growth card has the link
    const snapshotLink = page.getByRole('link', { name: 'Weekly Snapshot Share 📸' });
    await expect(snapshotLink).toBeVisible();

    // Click to navigate to the new page
    await snapshotLink.click();
    await page.waitForURL(/.*weekly-snapshot-share\.html/);

    // Verify loading state is initially present or that content loads
    await expect(page.locator('#snapshot-title')).toBeVisible();
    await expect(page.locator('#snapshot-title')).toHaveText('AI Weekly Snapshot Share');

    // Verify the data loads correctly from the API (mocking is forbidden, so we rely on actual API data)
    // The API responds with "124" and "$124,500" for the stats
    await expect(page.locator('#hours-saved')).toHaveText('124');
    await expect(page.locator('#total-sales')).toHaveText('$124,500');

    // Mock window.open to intercept the Share on X intent
    await page.evaluate(() => {
        (window as any).lastOpenedUrl = null;
        window.open = function(url: string | URL | undefined, target?: string, features?: string) {
            (window as any).lastOpenedUrl = url;
            return window;
        };
    });

    // Check share button
    const shareBtn = page.locator('#share-btn');
    await expect(shareBtn).toBeVisible();
    await expect(shareBtn).toBeEnabled();

    // Click share button
    await shareBtn.click();

    // Verify window.open was called with twitter intent and correct text
    const lastOpenedUrl = await page.evaluate(() => (window as any).lastOpenedUrl);
    expect(lastOpenedUrl).toContain('twitter.com/intent/tweet');
    expect(lastOpenedUrl).toContain('124%20hours');
    expect(lastOpenedUrl).toContain('%24124%2C500');
    expect(lastOpenedUrl).toContain('Powered%20by%20OHC');
  });
});

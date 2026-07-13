import { test, expect } from '../../../../e2e/fixtures';

test.describe('Interactive Trial Extension', () => {
  test('should allow user to claim 7 extra days of pro by sharing on X', async ({ page, context }) => {
    // Navigate to the dashboard first to simulate login and auth, setting up localStorage and cookies
    await page.goto('/dashboard');

    // Evaluate to set a fake auth state if necessary, since real auth involves Google/GitHub usually
    await page.evaluate(() => {
        localStorage.setItem('has_onboarded', 'true');
        localStorage.setItem('token', 'fake-token-for-test');
    });

    // Go to the trial extension page
    await page.goto('/trial-extension');

    // Check that the title exists
    await expect(page.locator('h1')).toHaveText('Interactive Trial Extension');
    await expect(page.locator('text=Want 7 Extra Days of Pro?')).toBeVisible();

    // Setup to handle new pages (window.open for X/Twitter)
    const pagePromise = context.waitForEvent('page');

    // Setup API mock since we shouldn't hit real external Twitter APIs and might need local auth mocked
    await page.route('/api/v1/growth/trial-extension/claim', async (route) => {
        await route.fulfill({
            status: 200,
            json: { success: true, message: 'Trial successfully extended to pro' },
        });
    });

    // Click the share button
    await page.click('button:has-text("Share on X to Unlock 7 Days")');

    // Wait for the new page to open
    const newPage = await pagePromise;
    expect(newPage.url()).toContain('twitter.com/intent/tweet');
    await newPage.close();

    // Verify that the success message appears
    await expect(page.locator('text=Trial Extended!')).toBeVisible();
    await expect(page.locator('text=Your Pro trial has been successfully extended by 7 days')).toBeVisible();
  });
});

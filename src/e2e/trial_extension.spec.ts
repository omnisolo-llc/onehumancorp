import { test, expect } from './fixtures';

test.describe('Trial Extension Growth Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('should display the trial extension widget and extend trial on share', async ({ page }) => {
    const dashboard = page.locator('#dashboard-screen');
    await expect(dashboard).toBeVisible();

    // Verify banner is visible
    const banner = page.locator('#trial-extension-banner');
    await expect(banner).toBeVisible();
    await expect(banner.getByRole('heading', { name: 'Extend Your Trial' })).toBeVisible();

    // Verify success message is hidden initially
    const successMsg = page.locator('#trial-extension-success');
    await expect(successMsg).toBeHidden();

    // Setup network interception to verify API is called
    let apiCalled = false;
    await page.route('/api/v1/growth/trial-extension/share', async (route) => {
      apiCalled = true;
      const request = route.request();
      expect(request.method()).toBe('POST');

      const postData = JSON.parse(request.postData() || '{}');
      expect(postData.platform).toBe('twitter');

      // Fulfill with success
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ extended: true, days_added: 14, message: "Trial successfully extended by 14 days for sharing on twitter!" }),
      });
    });

    // Click the share button
    const shareBtn = banner.getByRole('button', { name: 'Share to Extend' });
    await expect(shareBtn).toBeVisible();
    await shareBtn.click();

    // Verify success state
    await expect(banner).toBeHidden();
    await expect(successMsg).toBeVisible();
    await expect(successMsg).toContainText('Trial extended by 14 days!');

    // Verify API was actually intercepted and called
    expect(apiCalled).toBe(true);
  });
});

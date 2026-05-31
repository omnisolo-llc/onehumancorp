import { test, expect } from './fixtures';

test.describe('Viral Post-Purchase Growth Loop', () => {
  test('should track post-purchase share events', async ({ page }) => {
    await page.goto('/checkout');
    await page.waitForLoadState('networkidle');

    const payButton = page.getByRole('button', { name: 'Pay Now' });
    await payButton.click();

    // Verify the "Or Share Via" section exists
    const shareViaText = page.getByText('Or Share Via', { exact: true });
    await expect(shareViaText).toBeVisible();

    // Intercept the tracking API
    let trackedPlatform = '';
    await page.route('**/api/v1/growth/post-purchase-share', async route => {
      const request = route.request();
      if (request.method() === 'POST') {
        const body = JSON.parse(request.postData() || '{}');
        trackedPlatform = body.platform;
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ success: true, reward: 10 }),
        });
      } else {
        await route.continue();
      }
    });

    // Check for WhatsApp share button and click
    const whatsappButton = page.getByRole('link', { name: 'WhatsApp' });

    // Create a promise to wait for the API request
    const requestPromise = page.waitForRequest(req => req.url().includes('/api/v1/growth/post-purchase-share') && req.method() === 'POST');

    await whatsappButton.click();

    const request = await requestPromise;
    const postData = JSON.parse(request.postData() || '{}');
    expect(postData.platform).toBe('whatsapp');
  });
});
